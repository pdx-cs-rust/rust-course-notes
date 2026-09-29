SHELL := /bin/bash

PANDOC_VERSION := 3.11
PANDOC_SHA256 := 37edb3bbcf722f921a009941bf5874e2e0c09263226c9b4a2d980788cb062ab6
PANDOC_URL := https://github.com/jgm/pandoc/releases/download/$(PANDOC_VERSION)/pandoc-$(PANDOC_VERSION)-linux-amd64.tar.gz
LOCAL_PANDOC := .tools/bin/pandoc
PANDOC ?= $(if $(wildcard $(LOCAL_PANDOC)),$(LOCAL_PANDOC),pandoc)
BUILD_DIR ?= build/html
MANIFEST := canvas-notes.tsv

.PHONY: all html check clean install-pandoc

all: check

html:
	@set -euo pipefail; \
	command -v "$(PANDOC)" >/dev/null || { echo "pandoc not found; run 'make install-pandoc'" >&2; exit 1; }; \
	mkdir -p "$(BUILD_DIR)"; \
	while IFS=$$'\t' read -r source output; do \
		title=$$(awk '/^#+ / { sub(/^#+ /, ""); print; exit }' "$$source"); \
		"$(PANDOC)" --from=markdown --to=html5 \
			--standalone --metadata=pagetitle:"$$title" \
			--output="$(BUILD_DIR)/$$output" "$$source"; \
	done < "$(MANIFEST)"

check: html
	@set -euo pipefail; \
	expected=$$(wc -l < "$(MANIFEST)"); \
	actual=$$(find "$(BUILD_DIR)" -maxdepth 1 -type f -name '*.html' | wc -l); \
	test "$$actual" -eq "$$expected"; \
	test "$$(cut -f1 "$(MANIFEST)" | sort -u | wc -l)" -eq "$$expected"; \
	test "$$(cut -f2 "$(MANIFEST)" | sort -u | wc -l)" -eq "$$expected"; \
	test "$$(cut -f1 "$(MANIFEST)" | sort)" = "$$(find [0-9][0-9]-* -type f -name '*.md' | sort)"
	@set -euo pipefail; \
	while IFS=$$'\t' read -r source output; do \
		test -s "$(BUILD_DIR)/$$output"; \
		rg --quiet '^<!DOCTYPE html>' "$(BUILD_DIR)/$$output"; \
		rg --quiet '<meta charset="utf-8"' "$(BUILD_DIR)/$$output"; \
		"$(PANDOC)" --from=html --to=plain \
			--output=/dev/null "$(BUILD_DIR)/$$output"; \
	done < "$(MANIFEST)"

install-pandoc: $(LOCAL_PANDOC)

$(LOCAL_PANDOC):
	@mkdir -p .tools
	@curl -L --fail --show-error --output .tools/pandoc.tar.gz "$(PANDOC_URL)"
	@echo "$(PANDOC_SHA256)  .tools/pandoc.tar.gz" | sha256sum --check --status
	@tar -xzf .tools/pandoc.tar.gz --strip-components=1 -C .tools
	@rm .tools/pandoc.tar.gz

clean:
	@rm -rf build
