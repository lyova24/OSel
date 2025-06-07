run-bios:
	@cargo run --bin OSel-bios --features bios
test-bios:
	@cargo test --features bios


run-uefi:
	@cargo run --bin OSel-uefi --features uefi
test-uefi:
	@cargo test --features uefi
