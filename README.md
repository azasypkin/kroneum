# 🕐 Kroneum

**Kroneum** is an experimental, accessible (no GUI) and fully open source (both in code and hardware) time tracker device.

**Disclaimer:** I needed a dumb, autonomous, configurable and GUI-less physical timer to better manage my own time during the day and that is how Kroneum was born. 
Another goal was to see how well Rust fits into embedded development (basic interactions with MCU, USB HID and I2C drivers etc.).

There is neither GUI/LCD nor Wi-Fi/Bluetooth interface available, just two buttons one can use to configure the timer. More advanced users can use built-in USB functionality
to configure device (via dedicated [CLI tool](./sw/cli)), upgrade firmware (via [DFU interface](./sw/firmware/README.md)) or upload various Krouneum "recipes" (e.g. to repurpose device completely).

## Usage

By default device stays in a standby low power mode and wakes up as soon as any of the button is being pressed for 3-5 seconds. As mentioned above there are just two buttons: **Ⅰ** (Roman `one`) and **Ⅹ** (Roman `ten`).

* Long press on **Ⅰ** or **Ⅹ** button when in `StandBy` mode - device enters into `Setup` mode
* Short press on **Ⅰ** when in `Setup` mode - increases desired timer by one `unit`
* Short press on **Ⅹ** when in `Setup` mode - increases desired timer by ten `units`
* Long press on **Ⅰ** when in `Setup` mode - sets timer treating `unit` as a `second` (see example below)
* Long press on **Ⅹ** when in `Setup` mode - sets timer treating `unit` as a `minute` (see example below)
* Long press on both **Ⅰ** *and* **Ⅹ** when in `Setup` mode - sets timer treating `unit` as an `hour` (see example below)
* Long press on **Ⅰ** *or* **Ⅹ** when in `Alarm` mode - resets current alarm if any and enters `Setup` mode
* Long press on both **Ⅰ** *and* **Ⅹ** when in `Alarm` mode - resets current alarm if any and enters `StandBy` mode
* **Very** long press (5 seconds) on both **Ⅰ** *and* **Ⅹ** - enters `Configuration` mode and powers up USB interface

Once timer fires up it will be repeated every 10 seconds (configurable) until it's acknowledged by the long press on both **Ⅰ** *and* **Ⅹ**.

## Examples:

* Long Press on **Ⅰ** + 5 short presses on **Ⅰ** + long press on **Ⅰ** = `5s` timer
* Long Press on **Ⅰ** + 5 short presses on **Ⅰ** + long press on **Ⅹ** = `5m` timer
* Long Press on **Ⅰ** + 5 short presses on **Ⅰ** + long press on both **Ⅰ** *and* **Ⅹ** = `5h` timer
* Long Press on **Ⅰ** + 1 short press on **Ⅰ** + 2 short presses on *Ⅹ** + long press on **Ⅰ** = `21s` timer
* Long Press on **Ⅰ** + 1 short press on **Ⅹ** + long press on **Ⅹ** = `10m` timer
* and so on

## Configuration

Advanced users can configure device via USB with the help of dedicated [CLI tool](./sw/cli), assuming device is in `Configuration` mode and
connected to the host PC via micro USB cable:

```bash
$ cargo run -- info
$ cargo run -- alarm set "5m 15s"
```

## Prototype or DIY

Schematics is done in `KiCad` and can be found [here](./hw/pcb/Rev_0.3). PCB includes SWD, I2C and CR2032 connectors and may look a bit
oversized because of that, but on the bright side hand soldering and [CNC engraving](./hw/pcb/Rev_0.3/cnc) was a breeze.

Enclosure was machined from 7mm Plexiglas, G-Code can be found [here](./hw/pcb/Rev_0.3/cnc).

Rendered [PCB](./hw/pcb/Rev_0.3/demo/rendered.jpg) and [enclosure](./hw/enclosure/Rev_0.3/demo/rendered.png):


Machined [PCB](./hw/pcb/Rev_0.3/demo/cnc-manufactured.jpg) and [enclosure](./hw/enclosure/Rev_0.3/demo/cnc-manufactured.jpg):


-.- .-. --- -. . ..- --

