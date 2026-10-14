#!/usr/bin/env bash
set -euo pipefail

new_version="${1:?usage: scripts/bump-version.sh <new-version>}"
if ! echo "$new_version" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
  echo "::error::'${new_version}' doesn't look like a semver version (X.Y.Z)" >&2
  exit 1
fi

cd "$(git rev-parse --show-toplevel)"

old_version="$(awk '/^\[package\]/{f=1;next} /^\[/{f=0} f && /^version = /{gsub(/version = "|"/,""); print; exit}' Cargo.toml)"
if [ -z "$old_version" ]; then
  echo "::error::Couldn't find [package].version in Cargo.toml" >&2
  exit 1
fi

awk -v old="$old_version" -v new="$new_version" '
  /^\[package\]/ {f=1; print; next}
  /^\[/ {f=0}
  f && !done && $0 == "version = \"" old "\"" {print "version = \"" new "\""; done=1; next}
  {print}
' Cargo.toml > Cargo.toml.new
mv Cargo.toml.new Cargo.toml
grep -qxF "version = \"${new_version}\"" Cargo.toml || {
  echo "::error::Expected version = \"${new_version}\" in Cargo.toml after bump" >&2
  exit 1
}

sed -i.bak "s/^version = \"${old_version}\"/version = \"${new_version}\"/" python/Cargo.toml
sed -i.bak "s/libsonic = \"${old_version%.*}\"/libsonic = \"${new_version%.*}\"/" README.md
rm -f README.md.bak python/Cargo.toml.bak

cargo check --quiet
echo "Bumped ${old_version} -> ${new_version}"
git diff --stat -- Cargo.toml Cargo.lock README.md python/Cargo.toml
