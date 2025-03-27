idf_component_register(
    SRCS "nmea_idf.c"
    INCLUDE_DIRS "include"
)

# Define the Rust target for the Xtensa and RISC-V architectures
if (CONFIG_IDF_TARGET_ARCH_XTENSA)
    set(RUST_CARGO_TOOLCHAIN "+esp")
    set(RUST_CARGO_TARGET "xtensa-${IDF_TARGET}-none-elf")
elseif (CONFIG_IDF_TARGET_ARCH_RISCV)
    set(RUST_CARGO_TOOLCHAIN "+nightly")
    set(RUST_CARGO_TARGET "riscv32imac-unknown-none-elf")
else()
    message(FATAL_ERROR "Architecture currently not supported")
endif()

# Set the flags for cargo build (split into two separate arguments)
set(CARGO_BUILD_FLAGS "-Zbuild-std=core,panic_abort" "-Zbuild-std-features=panic_immediate_abort")

# Set directories and target
get_filename_component(RUST_PROJECT_DIR "${CMAKE_CURRENT_LIST_DIR}/nmea_idf_rs" ABSOLUTE)

set(RUST_BUILD_DIR "${CMAKE_CURRENT_BINARY_DIR}")
set(RUST_TARGET_DIR "${RUST_BUILD_DIR}/target")
set(RUST_STATIC_LIBRARY "${RUST_TARGET_DIR}/${RUST_CARGO_TARGET}/debug/libnmea_idf_rs.a")
# Uncomment the following line to use the release build:
# set(RUST_STATIC_LIBRARY "${RUST_TARGET_DIR}/${RUST_CARGO_TARGET}/release/libnmea_idf_rs.a")

# Use a dedicated prefix for ExternalProject_Add (do not use the source dir as prefix)
set(RUST_EXTERNAL_PREFIX "${RUST_BUILD_DIR}/rust_crate_target")

ExternalProject_Add(
    rust_crate_target
    PREFIX "${RUST_EXTERNAL_PREFIX}"
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND ${CMAKE_COMMAND} -E env
        CARGO_BUILD_TARGET=${RUST_CARGO_TARGET}
        CARGO_BUILD_TARGET_DIR=${RUST_TARGET_DIR}
        cargo ${RUST_CARGO_TOOLCHAIN} build ${CARGO_BUILD_FLAGS}
    BUILD_ALWAYS TRUE
    INSTALL_COMMAND ""
    TMP_DIR "${RUST_BUILD_DIR}/tmp"
    STAMP_DIR "${RUST_BUILD_DIR}/stamp"
    DOWNLOAD_DIR "${RUST_BUILD_DIR}"
    SOURCE_DIR "${RUST_PROJECT_DIR}"
    BINARY_DIR "${RUST_PROJECT_DIR}"
    INSTALL_DIR "${RUST_BUILD_DIR}"
    BUILD_BYPRODUCTS "${RUST_STATIC_LIBRARY}"
)

add_prebuilt_library(rust_crate_lib "${RUST_STATIC_LIBRARY}" REQUIRES "")

# Add dependency and link the Rust library with this component
add_dependencies(${COMPONENT_LIB} rust_crate_target)
target_link_libraries(${COMPONENT_LIB} PUBLIC rust_crate_lib)

