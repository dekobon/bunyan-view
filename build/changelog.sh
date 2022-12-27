#!/usr/bin/env bash

# This script generates a changelog for the current version of the project.

set -o errexit  # abort on nonzero exit status
set -o nounset  # abort on unbound variable
set -o pipefail # don't hide errors within pipes

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

pushd . > /dev/null
cd "${SCRIPT_DIR}/.."

if command -v ggrep > /dev/null; then
  GREP=ggrep
else
  GREP=grep
fi
if command -v gsed > /dev/null; then
  SED=gsed
else
  SED=sed
fi

# if gh is installed, use it to pull the last version number
if command -v gh > /dev/null; then
  LAST_RELEASE="$(gh release list | ${GREP} -E 'v[0-9]+\.[0-9]+\.[0-9]+' | cut -f1 | ${GREP} -v "${VERSION}" | head -n1)"
else
  LAST_RELEASE="$(git tag -l | ${GREP} -E '^v[0-9]+\.[0-9]+\.[0-9]+$$' | sort --version-sort --field-separator=. --reverse | ${GREP} -v "${VERSION}" | head -n1)"
fi

LAST_RELEASE_HASH="$(git show --format=%H "${LAST_RELEASE}" | head -n1 | ${SED} -e 's/^tag //')"

echo "## Changes between ${LAST_RELEASE} [$LAST_RELEASE_HASH] and ${VERSION}:"
git log --format="%s	(%h)" "${LAST_RELEASE_HASH}..HEAD" | \
  ${GREP} -E -v '^(ci|chore): .*' | \
  ${SED} 's/: /:\t/g1' | \
  column -s "	" -t | \
  ${SED} -e 's/^/ * /'

echo ""
popd > /dev/null