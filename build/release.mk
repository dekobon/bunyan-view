LAST_VERSION       = $(shell git tag -l | $(GREP) -E '^v[0-9]+\.[0-9]+\.[0-9]+$$' | sort --version-sort --field-separator=. --reverse | head -n1)
LAST_VERSION_HASH  = $(shell git show --format=%H $(LAST_VERSION) | head -n1)
CHANGES            = $(shell git log --format="%s	(%h)" "$(LAST_VERSION_HASH)..HEAD" | \
					 	$(GREP) -v '^(ci|chore): .*' | \
                        $(SED) 's/: /:\t/g1' | \
                        column -s "	" -t | \
                        $(SED) -e 's/^/ * /' | \
                        tr '\n' '\1')

.PHONY: changelog
.ONESHELL: changelog
changelog: ## Outputs the changes since the last version committed
	$Q echo 'Changes since $(LAST_VERSION):'
	$Q echo "$(CHANGES)" | tr '\1' '\n'

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