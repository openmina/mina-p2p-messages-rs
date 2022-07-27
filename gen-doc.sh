#!/bin/sh

set -e

CRATE=$(cargo metadata --format-version 1 | jq -r '.workspace_members[0] | split(" ") | .[0] | sub("-"; "_"; "g")')
rm -rf ./docs
cargo doc --no-deps
cp -r target/doc ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=$CRATE\">" > docs/index.html
