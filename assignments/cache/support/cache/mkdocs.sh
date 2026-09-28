#!/bin/sh
# https://dev.to/deciduously/prepare-your-rust-api-docs-for-github-pages-2n5i
cargo doc
rm -rf ./docs
cp -a target/doc ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=cache\">" \
  > docs/index.html
git add docs
