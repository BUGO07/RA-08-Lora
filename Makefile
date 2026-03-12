PYTHON           := python3
FLASHER          := flasher.py
SERIAL_PORT      ?= /dev/ttyUSB0
SERIAL_BAUDRATE  ?= 921600
FLASH_ADDRESS    ?= 0x08000000

CARGO_TARGET_DIR := target/thumbv7em-none-eabi/release
CARGO_ELF        := $(CARGO_TARGET_DIR)/ra08lora
CARGO_BIN        := $(CARGO_TARGET_DIR)/ra08lora.bin

OBJCOPY_FLAGS    := -O binary -R .eh_frame -R .init -R .fini -R .comment -R .ARM.attributes

.PHONY: all build flash clean clangdb

all: build

build:
	cargo build --release
	arm-none-eabi-objcopy $(OBJCOPY_FLAGS) $(CARGO_ELF) $(CARGO_BIN)
	arm-none-eabi-size $(CARGO_ELF)

flash: build
	echo Start flashing...
	sudo chmod a+rw $(SERIAL_PORT)
	$(PYTHON) $(FLASHER) -p $(SERIAL_PORT) -b $(SERIAL_BAUDRATE) flash $(FLASH_ADDRESS) $(CARGO_BIN)

clean:
	cargo clean

clangdb:
	bear -- make build
