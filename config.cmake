
set(CARGO_COMMAND cargo)
set(CARGO_BUILD_TYPE release)
set(TARGET riscv64gc-unknown-none-elf)
set(CROSS_COMPILE_PREFIX riscv64-unknown-linux-gnu-)
set(OBJCOPY rust-objcopy --binary-architecture=riscv64)
set(CARGO_ARGS "--${CARGO_BUILD_TYPE}" "--target" "${TARGET}")