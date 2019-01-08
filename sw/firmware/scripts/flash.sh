#!/usr/bin/env bash

cargo build --release -p kroneum-bin && \
    $(find $(rustc --print sysroot) -name llvm-objcopy) ./target/thumbv6m-none-eabi/release/kroneum-bin -O binary ./target/fw-upload.bin && \
    dfu-util -a 0 -d 0483:df11 -s 0x08000000:leave -D ./target/fw-upload.bin