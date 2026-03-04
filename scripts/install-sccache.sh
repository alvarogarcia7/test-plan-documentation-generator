#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/logger.sh"

SCCACHE_VERSION="${SCCACHE_VERSION:-0.7.7}"
INSTALL_DIR="${INSTALL_DIR:-${HOME}/.cargo/bin}"

detect_platform() {
    local os arch
    
    os="$(uname -s)"
    arch="$(uname -m)"
    
    case "${os}" in
        Linux*)
            case "${arch}" in
                x86_64)
                    echo "x86_64-unknown-linux-musl"
                    ;;
                aarch64)
                    echo "aarch64-unknown-linux-musl"
                    ;;
                *)
                    log_error "Unsupported architecture: ${arch}"
                    return 1
                    ;;
            esac
            ;;
        Darwin*)
            case "${arch}" in
                x86_64)
                    echo "x86_64-apple-darwin"
                    ;;
                arm64)
                    echo "aarch64-apple-darwin"
                    ;;
                *)
                    log_error "Unsupported architecture: ${arch}"
                    return 1
                    ;;
            esac
            ;;
        *)
            log_error "Unsupported OS: ${os}"
            return 1
            ;;
    esac
}

main() {
    log_info "Installing sccache v${SCCACHE_VERSION}..."
    
    if command -v sccache &> /dev/null; then
        local current_version
        current_version="$(sccache --version | head -n1 | awk '{print $2}')"
        if [[ "${current_version}" == "${SCCACHE_VERSION}" ]]; then
            log_success "sccache v${SCCACHE_VERSION} is already installed"
            return 0
        else
            log_warning "sccache ${current_version} is installed, but ${SCCACHE_VERSION} is requested"
        fi
    fi
    
    local platform
    platform="$(detect_platform)"
    log_info "Detected platform: ${platform}"
    
    local download_url="https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/sccache-v${SCCACHE_VERSION}-${platform}.tar.gz"
    local temp_dir
    temp_dir="$(mktemp -d)"
    
    log_info "Downloading from ${download_url}..."
    if ! curl -L -f -o "${temp_dir}/sccache.tar.gz" "${download_url}"; then
        log_error "Failed to download sccache"
        rm -rf "${temp_dir}"
        return 1
    fi
    
    log_info "Extracting archive..."
    tar -xzf "${temp_dir}/sccache.tar.gz" -C "${temp_dir}"
    
    mkdir -p "${INSTALL_DIR}"
    
    local binary_path="${temp_dir}/sccache-v${SCCACHE_VERSION}-${platform}/sccache"
    if [[ ! -f "${binary_path}" ]]; then
        log_error "Binary not found at expected path: ${binary_path}"
        rm -rf "${temp_dir}"
        return 1
    fi
    
    log_info "Installing to ${INSTALL_DIR}/sccache..."
    mv "${binary_path}" "${INSTALL_DIR}/sccache"
    chmod +x "${INSTALL_DIR}/sccache"
    
    rm -rf "${temp_dir}"
    
    log_success "sccache v${SCCACHE_VERSION} installed successfully"
    
    if [[ ":${PATH}:" != *":${INSTALL_DIR}:"* ]]; then
        log_warning "${INSTALL_DIR} is not in PATH. Add it to use sccache:"
        log_warning "  export PATH=\"${INSTALL_DIR}:\${PATH}\""
    fi
    
    log_info "sccache version: $(${INSTALL_DIR}/sccache --version)"
}

main "$@"
