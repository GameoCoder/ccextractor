extern crate pkg_config;
use std::env;
use std::path::PathBuf;

fn main() {
    let mut allowlist_functions = Vec::new();
    allowlist_functions.extend_from_slice(&[
        ".*(?i)_?dtvcc_.*",
        "get_visible_.*",
        "get_fts",
        "printdata",
        "writercwtdata",
        "version",
        "set_binary_mode",
    ]);

    #[cfg(feature = "hardsubx_ocr")]
    allowlist_functions.extend_from_slice(&[
        "edit_distance",
        "convert_pts_to_.*",
        "av_rescale_q",
        "mprint",
    ]);

    let mut allowlist_types = Vec::new();
    allowlist_types.extend_from_slice(&[
        ".*(?i)_?dtvcc_.*",
        "encoder_ctx",
        "lib_cc_decode",
        "bitstream",
        "cc_subtitle",
        "ccx_output_format",
        "ccx_boundary_time",
        "gop_time_code",
        "ccx_common_timing_settings_t",
        "ccx_s_options",
        "ccx_s_teletext_config",
        "ccx_output_date_format",
        "ccx_encoding_type",
        "ccx_decoder_608_settings",
        "ccx_decoder_608_report",
        "uint8_t",
        "word_list",
    ]);

    #[cfg(feature = "hardsubx_ocr")]
    allowlist_types.extend_from_slice(&["AVRational", "AVPacket", "AVFrame"]);

    let mut builder = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h");

    // enable hardsubx if and only if the feature is on
    #[cfg(feature = "hardsubx_ocr")]
    {
        builder = builder.clang_arg("-DENABLE_HARDSUBX");
    }

    // Tell cargo to invalidate the built crate whenever any of the
    // included header files changed.
    builder = builder.parse_callbacks(Box::new(bindgen::CargoCallbacks));

    for type_name in allowlist_types {
        builder = builder.allowlist_type(type_name);
    }

    for fn_name in allowlist_functions {
        builder = builder.allowlist_function(fn_name);
    }

    for rust_enum in RUSTIFIED_ENUMS {
        builder = builder.rustified_enum(rust_enum);
    }

    let bindings = builder
        .derive_default(true)
        .no_default("dtvcc_pen_attribs|dtvcc_pen_color|dtvcc_symbol")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

const RUSTIFIED_ENUMS: &[&str] = &[
    "dtvcc_(window|pen)_.*",
    "ccx_output_format",
    "ccx_output_date_format",
];
