#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/logger.sh"

REMOTE="${REMOTE:-origin}"

usage() {
    cat <<'EOF'
Usage: release-tag.sh --version-file=KIND:PATH [--tag-suffix=SUFFIX] [--bump=LEVEL] [--push] [--dry-run]

Reads the version from a project file, optionally bumps it, and creates an
annotated git tag v<version>[-<suffix>].

Options:
  --version-file=KIND:PATH  KIND is one of:
                              npm   JSON file with a top-level "version" (package.json, package-lock.json)
                              rust  Cargo.toml (first version = "..." line)
                              java  build.gradle or build.gradle.kts (first version = '...' line)
  --tag-suffix=SUFFIX       Appended to the tag, e.g. rc1 gives v0.1.0-rc1
  --bump=LEVEL              patch, minor or major. Updates the version file, commits
                            the change, then tags the new version
  --push                    Push the commit (when bumped) and the tag to $REMOTE
  --dry-run                 Print what would happen without changing anything
  -h, --help                Show this help
EOF
}

die() {
    log_error "$*"
    exit 1
}

parse_args() {
    VERSION_FILE=""
    TAG_SUFFIX=""
    BUMP=""
    PUSH=false
    DRY_RUN=false

    for arg in "$@"; do
        case "${arg}" in
            --version-file=*) VERSION_FILE="${arg#*=}" ;;
            --tag-suffix=*) TAG_SUFFIX="${arg#*=}" ;;
            --bump=*) BUMP="${arg#*=}" ;;
            --push) PUSH=true ;;
            --dry-run) DRY_RUN=true ;;
            -h | --help)
                usage
                exit 0
                ;;
            *)
                usage >&2
                die "Unknown argument: ${arg}"
                ;;
        esac
    done

    [[ -n "${VERSION_FILE}" ]] || die "--version-file is required (e.g. rust:Cargo.toml)"
    [[ "${VERSION_FILE}" == *:* ]] || die "--version-file must be KIND:PATH, got '${VERSION_FILE}'"
    KIND="${VERSION_FILE%%:*}"
    FILE="${VERSION_FILE#*:}"
    [[ -f "${FILE}" ]] || die "Version file not found: ${FILE}"

    case "${KIND}" in
        npm | rust | java) ;;
        *) die "Unknown version file kind '${KIND}' (expected npm, rust or java)" ;;
    esac

    if [[ -n "${TAG_SUFFIX}" && ! "${TAG_SUFFIX}" =~ ^[A-Za-z0-9._-]+$ ]]; then
        die "Invalid --tag-suffix '${TAG_SUFFIX}' (allowed: letters, digits, '.', '_', '-')"
    fi

    if [[ -n "${BUMP}" && ! "${BUMP}" =~ ^(patch|minor|major)$ ]]; then
        die "Invalid --bump '${BUMP}' (expected patch, minor or major)"
    fi
}

read_version() {
    local version
    case "${KIND}" in
        npm)
            version="$(jq -r '.version // empty' "${FILE}")"
            ;;
        rust)
            version="$(sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "${FILE}" | head -n 1)"
            ;;
        java)
            version="$(sed -n "s/^[[:space:]]*version[[:space:]]*=\{0,1\}[[:space:]]*['\"]\([^'\"]*\)['\"].*/\1/p" "${FILE}" | head -n 1)"
            ;;
    esac

    [[ -n "${version}" ]] || die "No version found in ${FILE}"
    [[ "${version}" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || die "Version '${version}' in ${FILE} is not MAJOR.MINOR.PATCH"
    echo "${version}"
}

write_version() {
    local new="$1"
    case "${KIND}" in
        npm)
            sed -i "0,/^  \"version\"[[:space:]]*:/s/^  \"version\"[[:space:]]*:[[:space:]]*\"[^\"]*\"/  \"version\": \"${new}\"/" "${FILE}"
            ;;
        rust)
            sed -i "0,/^version[[:space:]]*=/s/^version[[:space:]]*=[[:space:]]*\"[^\"]*\"/version = \"${new}\"/" "${FILE}"
            ;;
        java)
            sed -i "0,/^[[:space:]]*version[[:space:]]*=\{0,1\}[[:space:]]*['\"]/s/\(^[[:space:]]*version[[:space:]]*=\{0,1\}[[:space:]]*\)\(['\"]\)[^'\"]*['\"]/\1\2${new}\2/" "${FILE}"
            ;;
    esac
}

bump_version() {
    local current="$1" level="$2" major minor patch
    IFS=. read -r major minor patch <<<"${current}"
    case "${level}" in
        patch) patch=$((patch + 1)) ;;
        minor)
            minor=$((minor + 1))
            patch=0
            ;;
        major)
            major=$((major + 1))
            minor=0
            patch=0
            ;;
    esac
    echo "${major}.${minor}.${patch}"
}

tag_exists() {
    local tag="$1"
    if git rev-parse -q --verify "refs/tags/${tag}" >/dev/null; then
        return 0
    fi
    if git remote get-url "${REMOTE}" >/dev/null 2>&1 &&
        git ls-remote --exit-code --tags "${REMOTE}" "refs/tags/${tag}" >/dev/null 2>&1; then
        return 0
    fi
    return 1
}

run() {
    if [[ "${DRY_RUN}" == "true" ]]; then
        log_info "[dry-run] $*"
    else
        "$@"
    fi
}

main() {
    parse_args "$@"

    local current version tag
    current="$(read_version)"
    version="${current}"
    if [[ -n "${BUMP}" ]]; then
        version="$(bump_version "${current}" "${BUMP}")"
    fi

    tag="v${version}"
    if [[ -n "${TAG_SUFFIX}" ]]; then
        tag="${tag}-${TAG_SUFFIX}"
    fi

    log_info "Version file: ${FILE} (${KIND})"
    log_info "Current version: ${current}"
    log_info "Tag: ${tag}"

    if tag_exists "${tag}"; then
        die "Tag ${tag} already exists locally or on ${REMOTE}"
    fi

    if [[ -n "${BUMP}" ]]; then
        if [[ "${DRY_RUN}" != "true" ]] && ! git diff-index --quiet HEAD --; then
            die "Working tree has uncommitted changes; commit or stash them before bumping"
        fi
        log_info "Bumping ${BUMP}: ${current} -> ${version}"
        run write_version "${version}"
        run git add "${FILE}"
        run git commit -m "Bump version to ${version}"
    fi

    run git tag -a "${tag}" -m "Release ${tag}"
    if [[ "${DRY_RUN}" == "true" ]]; then
        log_info "Dry run complete; nothing was changed"
    else
        log_success "Created tag ${tag}"
    fi

    if [[ "${PUSH}" == "true" ]]; then
        if [[ -n "${BUMP}" ]]; then
            run git push "${REMOTE}" HEAD
        fi
        run git push "${REMOTE}" "refs/tags/${tag}"
        if [[ "${DRY_RUN}" != "true" ]]; then
            log_success "Pushed ${tag} to ${REMOTE}"
        fi
    else
        log_info "Not pushed. To publish: git push ${REMOTE} ${BUMP:+HEAD }refs/tags/${tag}"
    fi
}

main "$@"
