.PHONY: build check ci clean fmt install install-rustfmt link lint release run test

BIN_NAME = stava
CARGO = $(shell which cargo)

build:
	@$(CARGO) build

check:
	$(CARGO) check --release

ci: install-rustfmt lint check test

clean:
	rm -rf ./target

fmt:
	@$(CARGO) fmt

install:
	@cp ./target/release/$(BIN_NAME) /usr/local/bin/$(BIN_NAME)

install-rustfmt:
	@rustup component add rustfmt-preview

link:
	@ln -sf ./target/debug/$(BIN_NAME) .

lint:
	cargo fmt --all -- --check

# TODO: In CI - verify that packaged
# .cargo file has reasonable size
package:
	@$(CARGO) package

publish:
	@$(CARGO) publish

release:
	@$(CARGO) build --release

run:
	@RUST_BACKTRACE=1 $(CARGO) run

test:
	@$(CARGO) test -- --nocapture
