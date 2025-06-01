# How to Build

## Native build (host target, e.g. Linux x86_64-unknown-linux-gnu):
cmake -B build
cmake --build build

## Cross-build example for Linux from any host:
cmake -B build -DRUST_TARGET=x86_64-unknown-linux-gnu
cmake --build build

## Cross-build for macOS Intel:
cmake -B build -DRUST_TARGET=x86_64-apple-darwin
cmake --build build

## Cross-build for Windows MSVC:
cmake -B build -DRUST_TARGET=x86_64-pc-windows-msvc
cmake --build build
