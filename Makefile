.PHONY: check ci fmt lint publish test

BIN_NAME = stava
CARGO = $(shell which cargo)

check:
	$(CARGO) check --release

ci: lint check test

fmt:
	@$(CARGO) fmt

lint:
	$(CARGO) fmt --all -- --check

publish:
	$(CARGO) publish

test:
	@$(CARGO) test --lib -- --nocapture
