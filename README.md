# Rust Port of the Ai-thinker RA-08 Lora SDK

## Ported:
### Core
- [Cortex](src/cortex/mod.rs)
- [System](src/cortex/system.rs)
- [ARM Functions & Assembly](src/cortex/func.rs)
### Peripherals:
- [Delay](src/peripherals/delay.rs)
- [Flash](src/peripherals/flash.rs)
- [GPIO](src/peripherals/gpio.rs)
- [PWR](src/peripherals/pwr.rs)
- [RCC](src/peripherals/rcc.rs)
- [REGS](src/peripherals/regs.rs)
- [UART](src/peripherals/uart.rs)
- [WDG](src/peripherals/wdg.rs)
### ETC
- [Rust-Style Print Macros](print.rs)

## Instructions to run on linux (Ubuntu)

```
sudo apt-get install clang gcc-arm-none-eabi newlib-arm-none-eabi git vim python python-pip bear
python3 -m pip config set global.break-system-packages true
pip install pyserial configparser

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add thumbv7em-none-eabi

git clone --depth=1 https://github.com/BUGO07/RA-08-Lora

cd RA-08-Lora
make flashrs
```