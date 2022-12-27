# Release Process

This document describes the process for releasing a new version of the
project.

## Prerequisites

- You must be a maintainer of the project.
- You must have installed GNU Make 4+.
- You must have committer rights.

## Release Process

1.  Create a new branch for the release, e.g. `release-v1.2.3`:
    `git checkout -b release-v1.2.3`
2.  Update the Cargo.toml version to the new version:
    `make version-release`
3.  Check in version changes: `git commit Cargo.toml -m "ci: updating version to 1.2.3"`
4.  Wait for CI to build, tag and perform the release.
5.  Pull the latest changes from origin because the CI will have pushed changes to the branch.
6.  Once the release is complete, rebase the release branch into master using the generated PR 
    and delete the release branch from origin.
7.  Increment the version to the next development version: `make version-update` and
    use a version number like 1.2.4-beta.
8.  Check in version changes: `git commit Cargo.toml -m "ci: updating version to 1.2.4-beta"`