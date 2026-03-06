use crate::peripherals::{
    gpio::{Gpio, GpioPin},
    regs::{GPIOA, GPIOD},
};

pub const CONFIG_LORA_RFSW_CTRL_GPIOX: &Gpio = &GPIOD;
pub const CONFIG_LORA_RFSW_CTRL_PIN: GpioPin = GpioPin::Pin11;

pub const CONFIG_LORA_RFSW_VDD_GPIOX: &Gpio = &GPIOA;
pub const CONFIG_LORA_RFSW_VDD_PIN: GpioPin = GpioPin::Pin10;
