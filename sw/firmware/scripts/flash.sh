#!/usr/bin/env bash

TARGET="thumbv6m-none-eabi"
PROJECT="kroneum-bin"
BUILD_COMMAND=""
BINARY_PATH=""

# If argument is passed we treat it as example name, otherwise flash main binary
if [ "$1" != "" ]; then
  BUILD_COMMAND="cargo build --target $TARGET -p $PROJECT --example $1 --release"
  BINARY_PATH="release/examples/$1"
else
  BUILD_COMMAND="cargo build --target $TARGET -p $PROJECT --release"
  BINARY_PATH="release/kroneum-bin"
fi

if ! $BUILD_COMMAND; then
    echo -e "\n\e[31mFailed to build required binary $1\e[0m\n"
    exit 1
else
    echo -e "\n\e[32mSuccessfully built binary $1\e[0m\n"
fi

if ! $(find "$(rustc --print sysroot)" -name llvm-objcopy) "./target/${TARGET}/${BINARY_PATH}" -O binary ./target/fw.bin; then
    echo -e "\n\e[31mFailed to copy binary to the target location\e[0m\n"
    exit 1
else
    echo -e "\n\e[32mSuccessfully copied binary to the target location\e[0m\n"
fi

if ! dfu-util -a 0 -d 0483:df11 -s 0x08000000:leave -D ./target/fw.bin; then
    echo -e "\n\e[31mFailed to flash binary to the device\e[0m\n"
    exit 1
else
    echo -e "\n\e[32mSuccessfully flashed binary to the device\e[0m\n"
fi