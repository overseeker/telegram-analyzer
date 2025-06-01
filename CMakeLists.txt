cmake_minimum_required(VERSION 3.15)
project(TelegramAnalyzer LANGUAGES NONE)

# ----------------------------------------------------------------------------
# 1) Optionally specify a Rust target triple at configure time:
#    e.g. -DRUST_TARGET=x86_64-unknown-linux-gnu
#    (Leave empty for host-native build.)
set(RUST_TARGET "" CACHE STRING "Rust target triple (e.g. x86_64-unknown-linux-gnu)")

# ----------------------------------------------------------------------------
# 2) Decide the Cargo build command based on RUST_TARGET:
if(RUST_TARGET STREQUAL "")
    set(CARGO_BUILD_COMMAND cargo build --release)
    set(BINARY_PATH "${CMAKE_CURRENT_SOURCE_DIR}/target/release/telegram-analyzer")
else()
    set(CARGO_BUILD_COMMAND cargo build --release --target ${RUST_TARGET})
    set(BINARY_PATH "${CMAKE_CURRENT_SOURCE_DIR}/target/${RUST_TARGET}/release/telegram-analyzer")
endif()

# ----------------------------------------------------------------------------
# 3) Define a custom target that always runs "cargo build":
add_custom_target(telegram_analyzer ALL
    COMMAND ${CARGO_BUILD_COMMAND}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Building TelegramAnalyzer via Cargo"
)

# ----------------------------------------------------------------------------
# 4) Installation rule (installs the built binary to ${CMAKE_INSTALL_PREFIX}/bin):
install(CODE "
    execute_process(COMMAND ${CARGO_BUILD_COMMAND})
    file(INSTALL DESTINATION \"\${CMAKE_INSTALL_PREFIX}/bin\" TYPE FILE FILES \"${BINARY_PATH}\")
    message(STATUS \"Installed telegram-analyzer to \${CMAKE_INSTALL_PREFIX}/bin\")
")
