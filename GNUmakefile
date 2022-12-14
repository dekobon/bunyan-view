MAKE_MAJOR_VER    := $(shell echo $(MAKE_VERSION) | cut -d'.' -f1)

ifneq ($(shell test $(MAKE_MAJOR_VER) -gt 3; echo $$?),0)
$(error Make version $(MAKE_VERSION) is not supported, please install GNU Make 4.x)
endif

COMMITSAR_DOCKER  := docker run --tty --rm --workdir /src -v "$(CURDIR):/src" aevea/commitsar
COMMITSAR		  ?= $(shell command -v commitsar 2> /dev/null)

GREP              ?= $(shell command -v ggrep 2> /dev/null || command -v grep 2> /dev/null)
SED               ?= $(shell command -v gsed 2> /dev/null || command -v sed 2> /dev/null)
AWK               ?= $(shell command -v gawk 2> /dev/null || command -v awk 2> /dev/null)
DEB_ARCH          := $(shell uname -m | $(SED) -e 's/x86_64/amd64/g' -e 's/i686/i386/g')
RPM_ARCH          := $(shell uname -m)
VERSION           ?= $(shell $(GREP) -Po '^version\s+=\s+"\K.*?(?=")' $(CURDIR)/Cargo.toml)
CARGO             := cargo
DOCKER            := docker

BUILD_FLAGS       += --bin bunyan

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

.PHONY: all
all: target/debug/bunyan

target/%/bunyan:
	$Q if [ ! -f "$(CURDIR)/$(@)" ]; then \
  		echo "$(M) building $(@) with flags [$(BUILD_FLAGS)]"; \
		$(CARGO) build $(BUILD_FLAGS); \
	fi

.PHONY: debug
debug: target/debug/bunyan ## Create debug build for current platform

.PHONY: release
release: BUILD_FLAGS += --release
release: target/release/bunyan ## Create release build for current platform

.PHONY: test
test: ## Run tests
	$Q cargo test --features dumb_terminal

target/man/bunyan.1.gz:
	$(info $(M) processing manpage)
	$Q mkdir -p target/man
	$Q cp man/bunyan.1 target/man/bunyan.1
	$Q $(SED) -i 's/%%VERSION%%/$(VERSION)/' target/man/bunyan.1
	$Q gzip target/man/bunyan.1

.PHONY: manpage
manpage: target/man/bunyan.1.gz ## Builds man page

.PHONY: install-packaging-deb
install-packaging-deb:
	$Q cargo install --quiet cargo-deb

.PHONY: install-packaging-rpm
install-packaging-rpm:
	$Q cargo install --quiet cargo-generate-rpm

.PHONY: install-packaging-tools
install-packaging-tools: ## Installs tools needed for building distributable packages
	$Q cargo install --quiet cargo-deb cargo-generate-rpm

target/debian/bunyan_view_%.deb: target/man/bunyan.1.gz
	$Q if [ ! -f "$(CURDIR)/$(@)" ]; then \
  		echo "$(M) building debian package: $(@)"; \
		cargo deb; \
	fi

.PHONY: debian-package
debian-package: install-packaging-deb release manpage target/debian/bunyan_view_$(VERSION)_$(DEB_ARCH).deb ## Creates a debian package for the current platform

target/generate-rpm/bunyan_view_%.rpm: target/man/bunyan.1.gz
	$Q if [ ! -f "$(CURDIR)/$(@)" ]; then \
  		echo "$(M) building rpm package: $(@)"; \
		cargo generate-rpm; \
	fi

.PHONY: rpm-package
rpm-package: install-packaging-rpm clean release manpage target/generate-rpm/bunyan_view_$(VERSION)_$(RPM_ARCH).rpm ## Creates a rpm package for the current platform

.PHONY: container-debian-build-image
container-debian-build-image: ## Builds a container image for building on Debian Linux
	$Q if [ "$$($(DOCKER) images --quiet --filter=reference=bunyan_view_debian_builder)" = "" ]; then \
  		echo "$(M) building debian linux docker build image: $(@)"; \
  		$(DOCKER) build -t bunyan_view_debian_builder -f Containerfile.debian .; \
  	fi

.PHONY: container-debian-package
container-debian-package: container-debian-build-image ## Builds a rpm package using a the Debian Linux container image
	$Q $(DOCKER) run --rm --tty --interactive --volume "$(CURDIR):/project" --workdir /project bunyan_view_debian_builder make debian-package

.PHONY: container-rocky-build-image
container-rocky-build-image: ## Builds a container image for building on Rocky Linux
	$Q if [ "$$($(DOCKER) images --quiet --filter=reference=bunyan_view_rocky_builder)" = "" ]; then \
  		echo "$(M) building rocky linux docker build image: $(@)"; \
  		$(DOCKER) build -t bunyan_view_rocky_builder -f Containerfile.rocky .; \
  	fi

.PHONY: container-rpm-package
container-rpm-package: container-rocky-build-image ## Builds a rpm package using a the Rocky Linux container image
	$Q $(DOCKER) run --rm --tty --interactive --volume "$(CURDIR):/project" --workdir /project bunyan_view_rocky_builder make rpm-package

.PHONY: container-test
container-test: container-debian-build-image ## Run tests inside container
	$Q $(DOCKER) run --rm --tty --interactive --volume "$(CURDIR):/project" --workdir /project bunyan_view_rocky_builder make test