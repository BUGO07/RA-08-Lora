#/bin/sh

export TREMO_SDK_PATH=$(pwd)

which arm-none-eabi-gcc >/dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "arm-none-eabi-gcc not found"
fi
