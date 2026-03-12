use std::{env, error::Error, path::PathBuf};

const SOURCES: &[&str] = &[
    "lora/driver/utilities.c",
    "lora/system/crypto/cmac.c",
    "lora/mac/LoRaMac.c",
    "lora/mac/LoRaMacClassB.c",
    "lora/mac/LoRaMacConfirmQueue.c",
    "lora/mac/LoRaMacCrypto.c",
    "lora/mac/region/Region.c",
    "lora/mac/region/RegionCommon.c",
    "lora/mac/region/RegionEU868.c",
];

const INCLUDES: &[&str] = &[
    "platform/CMSIS",
    "platform/common",
    "platform/system",
    "drivers/crypto/inc",
    "lora/driver",
    "lora/mac",
    "lora/mac/region",
    "lora/system",
    "lora/system/crypto",
    "lora/radio",
];

fn main() -> Result<(), Box<dyn Error>> {
    let out_path = PathBuf::from(env::var("OUT_DIR")?);

    bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(INCLUDES.iter().map(|s| format!("-I{s}")))
        .prepend_enum_name(false)
        .merge_extern_blocks(true)
        .fit_macro_constants(true)
        .clang_macro_fallback()
        .use_core()
        .derive_default(true)
        .derive_debug(true)
        .derive_copy(true)
        .generate_cstr(true)
        .generate()?
        .write_to_file(out_path.join("bindings.rs"))?;

    cc::Build::new()
        .compiler("arm-none-eabi-gcc")
        .files(SOURCES)
        .includes(INCLUDES)
        .define("CONFIG_DEBUG_UART", "UART0")
        .define("USE_MODEM_LORA", None)
        .define("REGION_EU868", None)
        .flags([
            "-Wall",
            "-O3",
            "-ffunction-sections",
            "-fdata-sections",
            "-mcpu=cortex-m4",
            "-mthumb",
            "-nostdlib",
            "-ffreestanding",
        ])
        .std("gnu99")
        .compile("ra08lora");

    println!("cargo:rustc-link-search=native=drivers/crypto/lib");
    println!("cargo:rustc-link-lib=static=crypto");

    Ok(())
}
