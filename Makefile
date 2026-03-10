PYTHON         := python3
TREMO_LOADER   := flasher.py
SERIAL_PORT    ?= /dev/ttyUSB0
SERIAL_BAUDRATE?= 921600
FLASH_ADDRESS  ?= 0x08000000

CARGO_TARGET_DIR := target/thumbv7em-none-eabi/release
CARGO_ELF      := $(CARGO_TARGET_DIR)/ra08lora
CARGO_BIN      := $(CARGO_TARGET_DIR)/ra08lora.bin

OBJCOPY_FLAGS  := -O binary -R .eh_frame -R .init -R .fini -R .comment -R .ARM.attributes

ifneq ($(VERBOSE),1)
V := @
else
V :=
endif

.PHONY: all build flash clean clangdb

all: build

build:
	$(V)cargo build --release
	$(V)arm-none-eabi-objcopy $(OBJCOPY_FLAGS) $(CARGO_ELF) $(CARGO_BIN)
	$(V)arm-none-eabi-size $(CARGO_ELF)

flash: build
	$(V)echo Start flashing...
	$(V)sudo chmod a+rw $(SERIAL_PORT)
	$(V)$(PYTHON) $(TREMO_LOADER) -p $(SERIAL_PORT) -b $(SERIAL_BAUDRATE) flash $(FLASH_ADDRESS) $(CARGO_BIN)

clean:
	$(V)cargo clean

clangdb:
	$(V)rm -rf $(OUT_DIR)
	$(V)bear -- make build
