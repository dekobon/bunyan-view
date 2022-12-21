.PHONY: container-debian-build-image
container-debian-build-image: ## Builds a container image for building on Debian Linux
	$Q if [ "$$($(DOCKER) images --quiet --filter=reference=bunyan_view_debian_builder)" = "" ]; then \
  		echo "$(M) building debian linux docker build image: $(@)"; \
  		$(DOCKER) build -t bunyan_view_debian_builder -f Containerfile.debian .; \
  	fi

.PHONY: container-deb-packages
container-deb-packages: container-debian-build-image ## Builds deb packages using a container image
	$Q $(DOCKER) run --rm --tty --interactive --volume "$(CURDIR):/project" --workdir /project bunyan_view_debian_builder make deb-packages
	# Reset permissions on the target directory to the current user
	if command -v id > /dev/null; then \
		$(DOCKER) run --rm --tty --interactive --volume "$(CURDIR):/project" --workdir /project bunyan_view_debian_builder chown --recursive "$(shell id -u):$(shell id -g)" /project/target
	fi

.PHONY: container-rpm-packages
container-rpm-packages: container-debian-build-image ## Builds a rpm packages using a container image
	$Q $(DOCKER) run --rm --tty --interactive --volume "$(CURDIR):/project" --workdir /project bunyan_view_debian_builder make rpm-packages
	# Reset permissions on the target directory to the current user
	if command -v id > /dev/null; then \
		$(DOCKER) run --rm --tty --interactive --volume "$(CURDIR):/project" --workdir /project bunyan_view_debian_builder chown --recursive "$(shell id -u):$(shell id -g)" /project/target
	fi

.PHONY: container-all-packages
container-all-packages: container-debian-build-image ## Builds all packages using a container image
	$Q $(DOCKER) run --rm --tty --interactive --volume "$(CURDIR):/project" --workdir /project bunyan_view_debian_builder make all-packages
	# Reset permissions on the target directory to the current user
	if command -v id > /dev/null; then \
		$(DOCKER) run --rm --tty --interactive --volume "$(CURDIR):/project" --workdir /project bunyan_view_debian_builder chown --recursive "$(shell id -u):$(shell id -g)" /project/target
	fi

.PHONY: container-test
container-test: container-debian-build-image ## Run tests inside container
	$Q $(DOCKER) run --rm --tty --interactive --volume "$(CURDIR):/project" --workdir /project bunyan_view_rocky_builder make test