CARGO ?= cargo
RUSTY_IDD ?= $(CARGO) run --bin rusty-idd --

.PHONY: build test fmt fmt-check lint audit validate manifest manifest-check ci install-hooks clean

build:
	$(CARGO) build --workspace --locked

test:
	$(CARGO) test --workspace --locked

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all -- --check

lint:
	$(CARGO) clippy --workspace --all-targets --all-features -- -D warnings

audit:
	cargo audit --deny warnings

validate:
	$(RUSTY_IDD) validate --workspace .

manifest:
	$(RUSTY_IDD) manifest --workspace . --out .idd/MANIFEST.tsv

manifest-check: manifest
	git diff --exit-code -- .idd/MANIFEST.tsv

ci: build test validate manifest-check fmt-check lint audit

install-hooks:
	git config core.hooksPath .githooks

clean:
	$(CARGO) clean
