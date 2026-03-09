
PROJECT := $(notdir $(CURDIR))
TREMO_SDK_PATH := $(abspath .)

$(PROJECT)_SOURCE := \
    $(TREMO_SDK_PATH)/lora/driver/utilities.c \
    $(TREMO_SDK_PATH)/lora/system/crypto/cmac.c \
    $(TREMO_SDK_PATH)/lora/mac/LoRaMac.c \
    $(TREMO_SDK_PATH)/lora/mac/LoRaMacClassB.c \
    $(TREMO_SDK_PATH)/lora/mac/LoRaMacConfirmQueue.c \
    $(TREMO_SDK_PATH)/lora/mac/LoRaMacCrypto.c \
    $(TREMO_SDK_PATH)/lora/mac/region/Region.c \
    $(TREMO_SDK_PATH)/lora/mac/region/RegionCommon.c \
    $(TREMO_SDK_PATH)/lora/mac/region/RegionEU868.c

$(PROJECT)_INC_PATH := \
    $(TREMO_SDK_PATH)/platform/CMSIS \
    $(TREMO_SDK_PATH)/platform/common \
    $(TREMO_SDK_PATH)/platform/system \
    $(TREMO_SDK_PATH)/drivers/crypto/inc \
    $(TREMO_SDK_PATH)/lora/driver/ \
    $(TREMO_SDK_PATH)/lora/mac/ \
    $(TREMO_SDK_PATH)/lora/mac/region \
    $(TREMO_SDK_PATH)/lora/system/ \
    $(TREMO_SDK_PATH)/lora/system/crypto/ \
    $(TREMO_SDK_PATH)/lora/radio/

$(PROJECT)_CFLAGS  := -Wall -Os -ffunction-sections -mfpu=fpv4-sp-d16 -mfloat-abi=softfp -fsingle-precision-constant -std=gnu99
$(PROJECT)_DEFINES := -DCONFIG_DEBUG_UART=UART0 -DUSE_MODEM_LORA -DREGION_EU868

$(PROJECT)_LDFLAGS := -Wl,--gc-sections

$(PROJECT)_LIBS := target/thumbv7em-none-eabi/release/libra08lora.a $(TREMO_SDK_PATH)/drivers/crypto/lib/libcrypto.a

$(PROJECT)_LINK_LD := cfg/gcc.ld

# please change the settings to download the app
#SERIAL_PORT        :=
#SERIAL_BAUDRATE    :=
#$(PROJECT)_ADDRESS :=

##################################################################################################

include $(TREMO_SDK_PATH)/build/make/common.mk

clangdb:
	rm -rf out
	bear -- make

buildrs:
	cargo build --release
	rm -rf out/RA-08-Lora.*

flashrs: buildrs flash
