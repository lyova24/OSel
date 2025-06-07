run-bios:
	@cargo run --bin OSel-bios --target x86_64-osel-bios.json --features bios

test-bios:
	@cargo test --features bios


run-uefi:
	@cargo run --bin OSel-uefi --target x86_64-unknown-uefi --features uefi

build-uefi:
	@cargo build --target x86_64-unknown-uefi

test-uefi:
	@cargo test --features uefi
