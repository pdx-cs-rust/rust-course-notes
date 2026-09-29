SHELL := /bin/bash

PANDOC_VERSION := 3.11
PANDOC_SHA256 := 37edb3bbcf722f921a009941bf5874e2e0c09263226c9b4a2d980788cb062ab6
PANDOC_URL := https://github.com/jgm/pandoc/releases/download/$(PANDOC_VERSION)/pandoc-$(PANDOC_VERSION)-linux-amd64.tar.gz
LOCAL_PANDOC := .tools/bin/pandoc
PANDOC ?= $(if $(wildcard $(LOCAL_PANDOC)),$(LOCAL_PANDOC),pandoc)
HTML_BUILD_DIR ?= build/html
PAGE_BUILD_DIR ?= build/canvas-pages
MANIFEST := canvas-notes.tsv

.PHONY: all html canvas-pages check clean install-pandoc

all: check

html:
	@set -euo pipefail; \
	command -v "$(PANDOC)" >/dev/null || { echo "pandoc not found; run 'make install-pandoc'" >&2; exit 1; }; \
	mkdir -p "$(HTML_BUILD_DIR)"; \
	while IFS=$$'\t' read -r source output; do \
		title=$$(awk '/^#+ / { sub(/^#+ /, ""); print; exit }' "$$source"); \
		"$(PANDOC)" --from=markdown --to=html5 \
			--math-method=mathml \
			--standalone --metadata=pagetitle:"$$title" \
			--output="$(HTML_BUILD_DIR)/$$output" "$$source"; \
	done < "$(MANIFEST)"

canvas-pages:
	@set -euo pipefail; \
	command -v "$(PANDOC)" >/dev/null || { echo "pandoc not found; run 'make install-pandoc'" >&2; exit 1; }; \
	mkdir -p "$(PAGE_BUILD_DIR)"; \
	while IFS=$$'\t' read -r source output; do \
		"$(PANDOC)" --from=markdown --to=html5 \
			--math-method=mathml --syntax-highlighting=none \
			--output="$(PAGE_BUILD_DIR)/$$output" "$$source"; \
	done < "$(MANIFEST)"

check: html canvas-pages
	@set -euo pipefail; \
	expected=$$(wc -l < "$(MANIFEST)"); \
	html_actual=$$(find "$(HTML_BUILD_DIR)" -maxdepth 1 -type f -name '*.html' | wc -l); \
	page_actual=$$(find "$(PAGE_BUILD_DIR)" -maxdepth 1 -type f -name '*.html' | wc -l); \
	test "$$html_actual" -eq "$$expected"; \
	test "$$page_actual" -eq "$$expected"; \
	test "$$(cut -f1 "$(MANIFEST)" | sort -u | wc -l)" -eq "$$expected"; \
	test "$$(cut -f2 "$(MANIFEST)" | sort -u | wc -l)" -eq "$$expected"; \
	test "$$(cut -f1 "$(MANIFEST)" | sort)" = "$$(find [0-9][0-9]-* -type f -name '*.md' | sort)"
	@set -euo pipefail; \
	while IFS=$$'\t' read -r source output; do \
		test -s "$(HTML_BUILD_DIR)/$$output"; \
		rg --quiet '^<!DOCTYPE html>' "$(HTML_BUILD_DIR)/$$output"; \
		rg --quiet '<meta charset="utf-8"' "$(HTML_BUILD_DIR)/$$output"; \
		"$(PANDOC)" --from=html --to=plain \
			--output=/dev/null "$(HTML_BUILD_DIR)/$$output"; \
		test -s "$(PAGE_BUILD_DIR)/$$output"; \
		! rg --quiet '<(html|head|body|style|script)([ >])' \
			"$(PAGE_BUILD_DIR)/$$output"; \
		iconv --from-code=UTF-8 --to-code=UTF-8 \
			"$(PAGE_BUILD_DIR)/$$output" >/dev/null; \
		"$(PANDOC)" --from=html --to=plain \
			--output=/dev/null "$(PAGE_BUILD_DIR)/$$output"; \
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
