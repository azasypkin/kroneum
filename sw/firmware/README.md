## Debugging

Firs of all read [The Embedded Rust Book/Hardware](https://rust-embedded.github.io/book/start/hardware.html)!

Will debug with the help of ST-Link included into `STM32F0DISCOVERY` evaluation board. To program and debug `stm32f042f4p6` 
with this board remove the 2 jumpers from CN2 (see [en.DM00050135.pdf](./docs/en.DM00050135.pdf), page 16):

| `STM32F0DISCOVERY` CN3/SWD connector | `STM32F042F4P6`           |
| ------------------------------------ | -------------------------:|
| 1 - VDD_TARGET - VDD from target MCU | 16 - VDD __and__ 5 - VDDA |
| 2 - SWCLK - SWD clock                | 20 - PA14                 |
| 3 - GND - Ground                     | 15 - GND                  |
| 4 - SWDIO - SWD data input/output    | 19 - PA13                 |
| 5 - NRST - RESET of target MCU       | 4 - NRST                  |
| 6 - SWO - Reserved                   | NC                        |

1. Add `udev` rule for the `STMicroelectronics ST-LINK/V2`:

```bash
$ sudo vim /etc/udev/rules.d/99-stlink.rules

-------
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3748", MODE="0666"
-------

$ sudo udevadm trigger
```

1. Build project with one of the following commands:
```bash
$ cargo build
$ cargo build --target=thumbv6m-none-eabi
$ cargo build --target=thumbv6m-none-eabi --release
$ cargo +nightly build --target=thumbv6m-none-eabi --release --features nightly
```
2. Run `openocd -f openocd.cfg`
3. In another terminal run `arm-none-eabi-gdb  -x openocd.gdb target/thumbv6m-none-eabi/release/kroneum-bin`

# Firmware Upload via USB DFU

| `Micro USB` | `STM32F042F4P6`        |
| ------------------------------------:|
| 1 - VBUS +  | NC                     |
| 2 - DATA -  | 09 - PA11 - USB DM     |
| 3 - DATA +  | 10 - PA12 - USB DP     |
| 4 - ID      | NC                     |
| 5 - GND     | 15 - GND               |

Pin 1 (PB8-BOOT0) should be connected to VDD\VDDA (+3.3V).

Also make sure you read documentation for [cargo-binutils](https://github.com/rust-embedded/cargo-binutils) and [dfu-util](http://dfu-util.sourceforge.net/dfuse.html).

1. Build binary with:
```bash
$ cargo build --release -p kroneum-bin
$ $(find $(rustc --print sysroot) -name llvm-objcopy) ./target/thumbv6m-none-eabi/release/kroneum-bin -O binary ./target/fw-upload.bin
Or call the following command from the `./bin` subfolder
$ cargo objcopy --bin kroneum-bin --release -- -O binary ../target/fw-upload.bin
```

2. Upload binary with:
```bash
$ dfu-util -a 0 -d 0483:df11 -s 0x08000000:leave -D ./target/fw-upload.bin
```

3. Backup binary with:
```bash
$ dfu-util -U fw-from-device.bin -a 0 -d 0483:df11
``` 

## Notes

1. Steps 1-3 above can be replaced with just `$ ./scripts/flash.sh`.

2. If binary is too large GDB may fail so try to use `--release` flag with `cargo build`.

3. To reload program on the MCU use `monitor reset halt`

4. RTC & Low Power modes: https://github.com/mattico/stm32f0-Discovery_Tools/blob/master/ST_Example_Projects/Projects/Peripheral_Examples/PWR_CurrentConsumption/stm32f0xx_lp_modes.c


## Useful links

1. [The Embedded Rust Book](https://rust-embedded.github.io/book)
2. [Firmware for Anne Pro Keyboard written in Rust](https://github.com/ah-/anne-key)
3. [USB HID device development on STM32 F042](http://andybrown.me.uk/2016/01/09/f042usbhid/)
4. [A development board for the STM32F042 TSSOP package](http://andybrown.me.uk/2015/10/31/stm32f042dev/) and [schematics](http://andybrown.me.uk/wp-content/images/stm32f042dev/schematic.pdf)
5. [STM32F042 Breakout](http://ebrombaugh.studionebula.com/embedded/stm32f042breakout/index.html)
6. [Tim's Open Micro USB](https://github.com/im-tomu)
7. [Online Circuit Simulator](http://www.falstad.com/circuit/circuitjs.html?startCircuit=lrc.txt)
8. [ifUSB repo](https://github.com/julbouln/ifusb) and [USB multi protocol interface tool](https://hackaday.io/project/14864-ifusb)