# AHT20 RUST DEMO

This repo is just a demo for the [sensor_lib_aht20](https://github.com/jake-g00dwin/sensor_lib_aht20) library that is targeted 
for the stm32f103c8 in the demo.

## Building

```sh
cargo-embed --release --chip STM32F103C8
probe-rs attach /dev/stlink* --chip STM32F103C8
```


