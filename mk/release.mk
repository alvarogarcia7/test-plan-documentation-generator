# ==============================================================================
# Release Targets
# ==============================================================================
# Wraps scripts/release-tag.sh. The version goals (version-patch, version-minor,
# version-major) live in the top-level Makefile and call release-run.
#
# Configuration (override on the command line or in the environment):
#   RELEASE_VERSION_FILE  KIND:PATH, KIND is npm, rust or java
#                         (default: rust:Cargo.toml)
#   RELEASE_TAG_SUFFIX    Optional tag suffix, e.g. rc1 gives v0.1.0-rc1
#   RELEASE_PUSH          Set to 1 to push the commit and tag to origin
#   RELEASE_SCRIPT        Path to the tagging script
#
# Example:
#   make version-minor RELEASE_TAG_SUFFIX=rc2 RELEASE_VERSION_FILE=npm:package.json
# ==============================================================================

RELEASE_VERSION_FILE ?= rust:Cargo.toml
RELEASE_TAG_SUFFIX ?=
RELEASE_PUSH ?= 0
RELEASE_SCRIPT ?= scripts/release-tag.sh

RELEASE_SUFFIX_FLAG := $(if $(RELEASE_TAG_SUFFIX),--tag-suffix=$(RELEASE_TAG_SUFFIX))
RELEASE_PUSH_FLAG := $(if $(filter 1 true yes,$(RELEASE_PUSH)),--push)

# $(1) holds extra script flags, such as --bump=patch
define release-run
	@chmod +x $(RELEASE_SCRIPT)
	$(RELEASE_SCRIPT) --version-file=$(RELEASE_VERSION_FILE) $(RELEASE_SUFFIX_FLAG) $(RELEASE_PUSH_FLAG) $(1)
endef

release-tag:
	$(call release-run,)
.PHONY: release-tag
