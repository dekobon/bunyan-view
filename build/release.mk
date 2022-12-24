LAST_VERSION       := $(shell git tag -l | $(GREP) -E '^v[0-9]+\.[0-9]+\.[0-9]+$$' | sort --version-sort --field-separator=. --reverse | head -n1 | $(SED) -e 's/^v//')
LAST_VERSION_HASH  := $(shell git show --format=%H v$(LAST_VERSION) | head -n1)
CHANGES            := $(shell git log --format="%s	(%h)" "$(LAST_VERSION_HASH)..HEAD" | \
					 	$(GREP) -v '^(ci|chore): .*' | \
                        $(SED) 's/: /:\t/g1' | \
                        column -s "	" -t | \
                        $(SED) -e 's/^/ * /' | \
                        tr '\n' '\1')

.PHONY: changelog
.ONESHELL: changelog
changelog: ## Outputs the changes since the last version committed
	$Q echo 'Changes in $(VERSION) since $(LAST_VERSION):'
	echo "$(CHANGES)" | tr '\1' '\n'

.ONESHELL: target/dist/release_notes.md
target/dist/release_notes.md: target/dist target/dist/SHA256SUMS
	$(info $(M) building release notes) @
	$Q echo 'Changes since last release:' > target/dist/release_notes.md
	$Q echo '```' >> target/dist/release_notes.md
	$Q echo "$(CHANGES)" | tr '\1' '\n' >> target/dist/release_notes.md
	$Q echo '```' >> target/dist/release_notes.md
	$Q echo 'SHA256 Checksums:' >> target/dist/release_notes.md
	$Q echo '```' >> target/dist/release_notes.md
	$Q cat target/dist/SHA256SUMS >> target/dist/release_notes.md
	$Q echo '```' >> target/dist/release_notes.md

.PHONY: release-notes
release-notes: target/dist/release_notes.md ## Build release notes

.PHONY: version
version: ## Outputs the current version
	$Q echo "Version: $(VERSION)"

.PHONY: version-update
.ONESHELL: version-update
version-update: ## Prompts for a new version
	$(info $(M) updating repository to new version) @
	$Q echo "  last committed version: $(LAST_VERSION)"
	$Q echo "  Cargo.toml file version : $(VERSION)"
	read -p "  Enter new version in the format (MAJOR.MINOR.PATCH): " version
	$Q echo "$$version" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+-?.*$$' || \
		(echo "invalid version identifier: $$version" && exit 1) && \
	$(SED) -i "s/^version\s*=.*$$/version = \"$$version\"/" $(CURDIR)/Cargo.toml
	@ VERSION=$(shell $(GREP) -Po '^version\s+=\s+"\K.*?(?=")' $(CURDIR)/Cargo.toml)

.PHONY: version-release
.ONESHELL: version-release
version-release: ## Change from a pre-release to full release version
	$Q echo "$(VERSION)" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+-beta$$' || \
		(echo "invalid version identifier - must contain suffix -beta: $(VERSION)" && exit 1)
	export NEW_VERSION="$(shell echo $(VERSION) | $(SED) -e 's/-beta$$//')"
	$(SED) -i "s/^version\s*=.*$$/version = \"$$NEW_VERSION\"/" $(CURDIR)/Cargo.toml
	@ VERSION=$(shell $(GREP) -Po '^version\s+=\s+"\K.*?(?=")' $(CURDIR)/Cargo.toml)