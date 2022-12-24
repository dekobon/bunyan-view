.PHONY: container-debian-build-image
.ONESHELL: container-debian-build-image
container-debian-build-image:
container-debian-build-image: ## Builds a container image for building on Debian Linux
	$Q echo "$(M) building debian linux docker build image: $(@)"
	$(DOCKER) buildx build $(DOCKER_BUILD_FLAGS) -t debian_builder -f Containerfile.debian $(CURDIR);

.PHONY: container-deb-packages
container-deb-packages: container-debian-build-image ## Builds deb packages using a container image
	$Q $(DOCKER) run --rm --volume "$(CURDIR):/project" --workdir /project debian_builder make deb-packages
	# Reset permissions on the target directory to the current user
	if command -v id > /dev/null; then \
		$(DOCKER) run --rm --volume "$(CURDIR):/project" --workdir /project debian_builder chown --recursive "$(shell id -u):$(shell id -g)" /project/target
	fi

.PHONY: container-rpm-packages
container-rpm-packages: container-debian-build-image ## Builds a rpm packages using a container image
	$Q $(DOCKER) run --rm --volume "$(CURDIR):/project" --workdir /project debian_builder make rpm-packages
	# Reset permissions on the target directory to the current user
	if command -v id > /dev/null; then \
		$(DOCKER) run --rm --volume "$(CURDIR):/project" --workdir /project debian_builder chown --recursive "$(shell id -u):$(shell id -g)" /project/target
	fi

.PHONY: container-all-packages
container-all-packages: container-debian-build-image ## Builds all packages using a container image
	$Q $(DOCKER) run --rm --volume "$(CURDIR):/project" --workdir /project debian_builder make all-packages
	# Reset permissions on the target directory to the current user
	if command -v id > /dev/null; then \
		$(DOCKER) run --rm --volume "$(CURDIR):/project" --workdir /project debian_builder chown --recursive "$(shell id -u):$(shell id -g)" /project/target
	fi

.PHONY: container-test
container-test: container-debian-build-image ## Run tests inside container
	$Q $(DOCKER) run --rm --volume "$(CURDIR):/project" --workdir /project debian_builder make test
	# Reset permissions on the target directory to the current user
	if command -v id > /dev/null; then \
		$(DOCKER) run --rm --volume "$(CURDIR):/project" --workdir /project debian_builder chown --recursive "$(shell id -u):$(shell id -g)" /project/target
	fi
