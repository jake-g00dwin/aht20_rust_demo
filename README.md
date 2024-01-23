# AHT20 RUST DEMO

This repo is just a demo for the [sensor_lib_aht20](https://github.com/Personal-Data-Acquisition/sensor_lib_aht20) library that is targeted 
for the stm32f103c8 in the demo.

## Hardware Details

The required componets are:
1. a stm32f103C8 or another compatible uC.
2. An AHT20 series sensor module.
3. jumper wires to connect the B6(SDL) & B7(SDA) pins.

The demo itself runs at 100Khz in standard i2c mode using 7bit addressing.


## Building

```sh
cargo-embed --release --chip STM32F103C8
probe-rs attach /dev/stlink* --chip STM32F103C8
```


