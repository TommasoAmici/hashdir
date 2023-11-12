build:
	cargo build --verbose --release

test_unit:
	cargo test --release

test_e2e:
	sh test_e2e.sh

test: test_unit test_e2e
