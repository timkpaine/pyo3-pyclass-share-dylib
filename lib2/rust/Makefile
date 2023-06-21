.DEFAULT_GOAL := help
.PHONY: dev build lint fix check tests tests-ci-run tests-ci-gha tests-ci-jenkins develop test format checks help

dev:  ## Install required dev dependencies
	rustup component add rustfmt
	rustup component add clippy
	cargo install cargo2junit
	cargo install grcov
	rustup component add llvm-tools-preview

build:  ## Build release
	cargo build --release --all-features \
		-p lib2 \
		-p lib2_py

lint:  ## Run Clippy for linting, rustfmt for autoformat checks
	cargo clippy --all-features \
		-p lib2 \
		-p lib2_py
	cargo fmt --all -- --check

fix:  ## Fix code with rustfmt
	cargo fmt --all \
		-p lib2 \
		-p lib2_py

check:
	cargo check --all-features \
		-p lib2 \
		-p lib2_py

tests:  ## Run the tests
	cargo test -- --show-output 
	# cargo test -- -Z unstable-options --format json | cargo2junit > results.xml

tests-ci: $(eval SHELL:=/bin/bash)
	{ \
		export CARGO_INCREMENTAL=0;\
		export RUSTDOCFLAGS="-Cpanic=abort";\
		export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort";\
		cargo test -- -Z unstable-options --format json | cargo2junit > junit.xml;\
		grcov . --llvm -s . -t cobertura --branch --ignore-not-existing -o ./coverage.xml;\
	}


# aliases
develop: dev
test: tests
format: fix
checks: check

# Thanks to Francoise at marmelab.com for this
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

print-%:
	@echo '$*=$($*)'
