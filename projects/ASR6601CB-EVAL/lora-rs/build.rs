use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let tremo_sdk_path = env::var("TREMO_SDK_PATH").unwrap();
    println!("{}", env::consts::OS);
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(&[
            &format!("-I{}/drivers/crypto/inc", tremo_sdk_path),
            &format!("-I{}/drivers/peripheral/inc", tremo_sdk_path),
            &format!("-I{}/platform/system", tremo_sdk_path),
            &format!("-I{}/platform/CMSIS", tremo_sdk_path),
            &format!("-I{}/lora/driver", tremo_sdk_path),
            &format!("-I{}/lora/linkwan/inc", tremo_sdk_path),
            &format!("-I{}/lora/linkwan/region", tremo_sdk_path),
            &format!("-I{}/lora/mac", tremo_sdk_path),
            &format!("-I{}/lora/mac/region", tremo_sdk_path),
            &format!("-I{}/lora/radio", tremo_sdk_path),
            &format!("-I{}/lora/radio/sx126x", tremo_sdk_path),
            &format!("-I{}/lora/system", tremo_sdk_path),
            &format!("-I{}/lora/system/cmac", tremo_sdk_path),
            "-Iinc",
            "-I/usr/arm-none-eabi/include",
            "-mfpu=none",
            "-mfloat-abi=softfp",
        ])
        .prepend_enum_name(false)
        .merge_extern_blocks(true)
        .fit_macro_constants(true)
        .clang_macro_fallback()
        .use_core()
        .derive_default(true)
        .derive_debug(true)
        .derive_copy(true)
        .generate_cstr(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}
