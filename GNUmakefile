MAKE_MAJOR_VER    := $(shell echo $(MAKE_VERSION) | cut -d'.' -f1)

ifneq ($(shell test $(MAKE_MAJOR_VER) -gt 3; echo $$?),0)
$(error Make version $(MAKE_VERSION) is not supported, please install GNU Make 4.x)
endif

GREP              ?= $(shell command -v ggrep 2> /dev/null || command -v grep 2> /dev/null)
SED               ?= $(shell command -v gsed 2> /dev/null || command -v sed 2> /dev/null)
AWK               ?= $(shell command -v gawk 2> /dev/null || command -v awk 2> /dev/null)
RUSTUP            ?= $(shell command -v rustup 2> /dev/null)
RPM_ARCH          := $(shell uname -m)
VERSION           ?= $(shell $(GREP) -Po '^version\s+=\s+"\K.*?(?=")' $(CURDIR)/Cargo.toml)
DEFAULT_TARGET    ?= $(shell $(RUSTUP) toolchain list | $(GREP) '(default)' | cut -d' ' -f1 | cut -d- -f2-)
SHELL             := /bin/bash
OUTPUT_BINARY     ?= bunyan
PACKAGE_NAME      ?= bunyan-view
CARGO             ?= cargo
DOCKER            ?= docker
CHECKSUM          ?= sha256sum
COMMITSAR_DOCKER  := $(DOCKER) run --tty --rm --workdir /src -v "$(CURDIR):/src" aevea/commitsar
COMMITSAR		  ?= $(shell command -v commitsar 2> /dev/null)

# Define platform targets based off of the current host OS
# If we are running MacOS, then we can build for MacOS platform targets that have been installed in rustup
# If we are running Linux, then we can build for Linux platform targets that have been installed in rustup
UNAME = $(shell uname | tr '[:upper:]' '[:lower:]')
ifeq ($(UNAME),darwin)
	TARGETS       := $(sort $(shell $(RUSTUP) target list | $(GREP) '(installed)' | $(GREP) 'apple' | cut -d' ' -f1))
else ifeq ($(UNAME),linux)
	TARGETS       := $(sort $(shell $(RUSTUP) target list | $(GREP) '(installed)' | $(GREP) 'linux' | cut -d' ' -f1))
else
	TARGETS       := $(DEFAULT_TARGET)
endif

RELEASE_BUILD_FLAGS ?= --quiet --release --bin $(OUTPUT_BINARY)

Q = $(if $(filter 1,$V),,@)
M = $(shell printf "\033[34;1m▶\033[0m")

# Use docker based commitsar if it isn't in the path
ifeq ($(COMMITSAR),)
	COMMITSAR = $(COMMITSAR_DOCKER)
endif

.PHONY: help
help:
	@grep --no-filename -E '^[ a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		$(AWK) 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-28s\033[0m %s\n", $$1, $$2}' | sort

.PHONY: clean
clean: ; $(info $(M) cleaning...)	@ ## Cleanup everything
	$Q rm -rf $(CURDIR)/target

.PHONY: commitsar
commitsar: ## Run git commit linter
	$Q $(info $(M) running commitsar...)
	$(COMMITSAR)

.PHONY: list-targets
list-targets: ## List all available platform targets
	$Q echo $(TARGETS) | $(SED) -e 's/ /\n/g'

.PHONY: all
all: $(TARGETS) ## Build all available platform targets [see: list-targets]

.PHONY: $(TARGETS)
.ONESHELL: $(TARGETS)
$(TARGETS): ## Build for a specific target
	$Q if [ ! -f "$(CURDIR)/target/$(@)/release/$(OUTPUT_BINARY)" ]; then
		echo "$(M) building $(OUTPUT_BINARY) with flags [$(RELEASE_BUILD_FLAGS) --target $(@)]"
		$(CARGO) build $(RELEASE_BUILD_FLAGS) --target $@
	fi

target:
	$Q mkdir -p $@

.PHONY: debug
debug: target/debug/$(OUTPUT_BINARY) ## Build current platform target in debug mode

target/debug/$(OUTPUT_BINARY):
	$Q echo "$(M) building $(OUTPUT_BINARY) in debug mode for the current platform"
	$Q $(CARGO) build --bin $(OUTPUT_BINARY)

.PHONY: release
release: target/release/$(OUTPUT_BINARY) ## Build current platform target in release mode

target/release/$(OUTPUT_BINARY):
	$Q echo "$(M) building $(OUTPUT_BINARY) in release mode for the current platform"
	$Q $(CARGO) build $(RELEASE_BUILD_FLAGS)

.PHONY: test
test: ## Run tests
	$Q $(CARGO) test --features dumb_terminal

.ONESHELL: target/man/$(OUTPUT_BINARY).1.gz
target/man/$(OUTPUT_BINARY).1.gz:
	$Q $(info $(M) building distributable manpage)
	mkdir -p target/man
	cp man/$(OUTPUT_BINARY).1 target/man/$(OUTPUT_BINARY).1
	$(SED) -i 's/%%VERSION%%/$(VERSION)/' target/man/$(OUTPUT_BINARY).1
	gzip target/man/$(OUTPUT_BINARY).1

target/gz:
	$Q mkdir -p target/gz

.PHONY: manpage
manpage: target/man/$(OUTPUT_BINARY).1.gz ## Builds man page

include $(CURDIR)/build/package.mk
include $(CURDIR)/build/container.mk