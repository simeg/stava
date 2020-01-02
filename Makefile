.PHONY: check ci fmt lint test

BIN_NAME = stava
CARGO = $(shell which cargo)

check:
	$(CARGO) check --release

ci: lint check test

fmt:
	@$(CARGO) fmt

lint:
	$(CARGO) fmt --all -- --check

test:
	@$(CARGO) test --lib -- --nocapture
