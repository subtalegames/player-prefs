set shell := ["pwsh.exe", "-c"]

fmt:
  cargo +nightly fmt

clippy:
  cargo clippy

lint: fmt clippy

test:
  cargo test --all-features

bench:
  cargo bench --all-features
