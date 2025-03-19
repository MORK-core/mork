cmake_minimum_required(VERSION 3.10)
project(MORK)

set(CARGO_COMMAND cargo)

set(CMAKE_BUILD_TYPE Release)

set(TARGET riscv64gc-unknown-none-elf)
set(CROSS_COMPILE_PREFIX riscv64-unknown-linux-gnu-)
set(OBJCOPY rust-objcopy --binary-architecture=riscv64)

set(OS_DIR ${CMAKE_CURRENT_SOURCE_DIR}/os)
set(LINKER_SCRIPT "${CMAKE_SOURCE_DIR}/os/linker-qemu.ld")
set(RUSTFLAGS "-C link-arg=-T${LINKER_SCRIPT} -C relocation-model=static")

set(CARGO_ARGS "--release" "--target" "${TARGET}")

add_custom_target(os_build ALL
    COMMAND ${CMAKE_COMMAND} -E env "RUSTFLAGS=${RUSTFLAGS}" ${CARGO_COMMAND} build ${CARGO_ARGS}
    WORKING_DIRECTORY ${OS_DIR}
    COMMENT "Building OS library"
)

set(OS_FILE ${OS_DIR}/target/${TARGET}/release/os)

add_custom_command(
    OUTPUT ${OS_FILE}
    COMMAND ${CMAKE_COMMAND} -E echo "Waiting for Rust library to build..."
    DEPENDS os_build
)

add_custom_command(
    OUTPUT ${OS_FILE}
    COMMAND ${CMAKE_COMMAND} -E echo "Waiting for Rust library to build..."
    DEPENDS os_build
)

add_custom_command(TARGET os_build POST_BUILD
    COMMAND ${OBJCOPY} --strip-all -O binary ${OS_FILE} ${OS_FILE}.bin
    COMMAND ${CMAKE_COMMAND} -E copy ${OS_FILE}.bin ${CMAKE_BINARY_DIR}
    COMMENT "Stripping symbols from my_app"
)


add_library(os STATIC IMPORTED)
set_target_properties(os PROPERTIES IMPORTED_LOCATION ${OS_FILE})

set(OPENSBI_DIR ${CMAKE_SOURCE_DIR}/opensbi)
set(OPENSBI_BUILD_COMMAND make CROSS_COMPILE=${CROSS_COMPILE_PREFIX} -C ${OPENSBI_DIR} PLATFORM=generic)

add_custom_target(
    build_opensbi ALL
    COMMAND ${OPENSBI_BUILD_COMMAND}
    WORKING_DIRECTORY ${OPENSBI_DIR}
    COMMENT "Building OpenSBI"
)

set(OPENSBI_OUTPUT ${OPENSBI_DIR}/build/platform/generic/firmware/fw_jump.bin)

add_custom_command(
    TARGET build_opensbi POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy ${OPENSBI_OUTPUT} ${CMAKE_BINARY_DIR}
    COMMENT "Copying OpenSBI output to build directory"
)