use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(&[
            "-Idrivers/crypto/inc",
            "-Iplatform/system",
            "-Iplatform/CMSIS",
            "-Ilora/driver",
            "-Ilora/mac",
            "-Ilora/mac/region",
            "-Ilora/radio",
            "-Ilora/system",
            "-Ilora/system/cmac",
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
        .generate()?
        .write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}
