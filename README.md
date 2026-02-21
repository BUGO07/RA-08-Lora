# Rust Port of the Ai-thinker RA-08 Lora SDK

## Ported:
### Peripherals:
- GPIO
- UART
- REGS
- RCC

## Instructions to run on linux (Ubuntu)

```
git clone https://github.com/BUGO07/RA-08-Lora
cd RA-08-Lora

sudo apt-get install clang gcc-arm-none-eabi newlib-arm-none-eabi git vim python python-pip
python3 -m pip config set global.break-system-packages true
pip install pyserial configparser

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add thumbv7em-none-eabi

cd lora-rs
make flashrs
```