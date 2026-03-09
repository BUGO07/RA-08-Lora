# Rust Port of the Ai-thinker RA-08 Lora SDK

## There's some unsafe rust code for C compatibility, once everything is ported, it will no longer be necessary, and will be removed.

## Ported:
- drivers/peripheral/*
- lora/driver/*
- lora/radio/*
- lora/system/*
### Core
- [Cortex](src/cortex/mod.rs)
- [System](src/cortex/system.rs)
- [ARM Functions](src/cortex/func.rs)
- [ARM Assembly](src/cortex/asm.rs)
- [Interrupts](src/interrupts.rs)
### Peripherals
- [Delay](src/peripherals/delay.rs)
- [Flash](src/peripherals/flash.rs)
- [GPIO](src/peripherals/gpio.rs)
- [I2C](src/peripherals/i2c.rs)
- [I2S](src/peripherals/i2s.rs)
- [IWDG](src/peripherals/iwdg.rs)
- [LCD](src/peripherals/lcd.rs)
- [LP Timer](src/peripherals/lptimer.rs)
- [LP UART](src/peripherals/lpuart.rs)
- [PWR](src/peripherals/pwr.rs)
- [RCC](src/peripherals/rcc.rs)
- [REGS](src/peripherals/regs.rs)
- [SPI](src/peripherals/spi.rs)
- [System](src/peripherals/system.rs)
- [Timer](src/peripherals/timer.rs)
- [UART](src/peripherals/uart.rs)
- [WDG](src/peripherals/wdg.rs)
### LoRa
- [SX126x Radio Driver](src/lora/radio/sx126x.rs)
- [LoRa Radio Driver](src/lora/radio/mod.rs)
- [SX1262 Board Driver](src/lora/driver/sx1262_board.rs)
- [RTC Board Driver](src/lora/driver/rtc_board.rs)
- [LoRa Timer](src/lora/timer.rs)
- [LoRa Config](src/lora_config.rs)
- [Class C Application](src/class_c.rs)
### ETC
- [Rust-Style Print Macros](src/print.rs)

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

## Docs:
- `cargo doc --release`
- Open the HTML file cargo-doc generates.