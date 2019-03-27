#![deny(rust_2018_idioms)]

use std::{env, path::Path};
#[cfg(all(windows, target_env = "msvc"))]
use std::path::PathBuf;

#[cfg(any(unix, target_env = "gnu"))]
use std::process::Command;

/// Outputs the library-file's prefix as word usable for actual arguments on
/// commands or paths.
fn rustc_linking_word(is_static_link: bool) -> &'static str {
    if is_static_link {
        "static"
    } else {
        "dylib"
    }
}

/// Builds Opus on Unix or GNU.
/// If we want to build for Window's GNU-toolchain, we need to build in MSYS2.
///
/// Building Opus consists of four steps:
/// 1. Run `autogen.sh`.
/// 2. Configure the generated file to prepare building.
/// 3. Building Opus.
/// 4. Installing the built Opus in `OUT_DIR`.
#[cfg(any(unix, target_env = "gnu"))]
fn build_opus(
    build_directory: &Path,
    is_static: bool,
    installed_lib_directory: &Option<String>,
) {
    let is_static_text = rustc_linking_word(is_static);

    if let Some(prebuilt_directory) = installed_lib_directory {
        println!(
            "{}",
            format!("cargo:rustc-link-lib={}=opus", is_static_text)
        );
        println!("cargo:rustc-link-search=native={}", prebuilt_directory);

        return;
    }

    let opus_path = Path::new("opus")
        .canonicalize()
        .expect("Could not canonicalise.");

    println!(
        "cargo:info=Opus source path: {:?}.",
        &opus_path.to_string_lossy()
    );
    println!(
        "cargo:info=Opus will be built as {}-library.",
        is_static_text
    );

    let command_result = Command::new("sh")
        .arg("autogen.sh")
        .current_dir(&opus_path)
        .status()
        .expect("Failed to run `sh autogen.sh`.");

    if !command_result.success() {
        panic!("Failed to autogen Opus.");
    }

    let mut command_builder = Command::new("sh");

    command_builder.arg("configure");

    if is_static {
        command_builder
            .arg("--enable-static")
            .arg("--disable-shared");
    } else {
        command_builder
            .arg("--disable-static")
            .arg("--enable-shared");
    }

    let command_result = command_builder
        .arg("--disable-doc")
        .arg("--disable-extra-programs")
        .arg("--with-pic")
        .arg("--prefix")
        .arg(
            build_directory
                .to_str()
                .expect("Build Path contains invalid characters.")
                .replace("\\", "/"),
        )
        .current_dir(&opus_path)
        .status()
        .expect("Failed to run `configure` Opus.");

    if !command_result.success() {
        panic!("Failed to configure Opus.");
    }

    let command_result = Command::new("make")
        .current_dir(&opus_path)
        .status()
        .expect("Failed to run `make`.");

    if !command_result.success() {
        panic!("Failed to build Opus via `make`.");
    }

    let command_result = Command::new("make")
        .current_dir(&opus_path)
        .arg("install")
        .status()
        .expect("Failed to run `make install`.");

    if !command_result.success() {
        panic!("Failed to install Opus via `make install`.");
    }

    let command_result = Command::new("make")
        .current_dir(&opus_path)
        .arg("clean")
        .status()
        .expect("Failed to run `make clean`.");

    if !command_result.success() {
        panic!("Failed to clean up build artefacts.");
    }

    println!("cargo:rustc-link-lib={}=opus", is_static_text);
    println!(
        "cargo:rustc-link-search=native={}/lib",
        build_directory.display()
    );
}

#[cfg(all(windows, target_env = "msvc"))]
fn build_opus(
    _build_directory: &Path,
    is_static: bool,
    installed_lib_directory: &Option<String>,
) {
    link_prebuilt_opus(is_static, installed_lib_directory);
}

/// Links to prebuilt Windows library-files of Opus.
#[cfg(all(windows, target_env = "msvc"))]
fn link_prebuilt_opus(is_static: bool, installed_lib_directory: &Option<String>) {
    let is_static_text = rustc_linking_word(is_static);

    #[cfg(target_arch = "x86")]
    const ARCHITECTURE: &'static str = "x86";
    #[cfg(target_arch = "x86_64")]
    const ARCHITECTURE: &'static str = "x64";

    const OPUS_DLL: &'static str = "opus.dll";

    if let Some(prebuilt_directory) = installed_lib_directory {
        println!(
            "cargo:info=Prebuilt Opus will be linked: {}",
            prebuilt_directory
        );

        println!(
            "{}",
            format!("cargo:rustc-link-lib={}=opus", is_static_text)
        );
        println!("cargo:rustc-link-search=native={}", prebuilt_directory);

        return;
    }

    let mut building_path = Path::new("msvc").join(ARCHITECTURE);

    if !is_static {
        building_path = building_path.join("dy");
    }

    let library_path = building_path
        .canonicalize()
        .expect("Could not canonicalise.");

    println!("cargo:info=Try to build {} library.", is_static_text);
    println!("cargo:rustc-link-lib={}=opus", is_static_text);
    println!("cargo:rustc-link-search=native={}", library_path.display());

    if !is_static {
        building_path = building_path.join(OPUS_DLL);

        let dll_destination = find_cargo_target_dir();
        let dll_destination = dll_destination.join(OPUS_DLL);

        println!(
            "cargo:info=Found Cargo target directory: {:?}.",
            &dll_destination
        );

        std::fs::copy(&building_path, &dll_destination).expect(&format!(
            "Failed to copy `opus.dll` from `{}` to `{}`.",
            building_path.to_string_lossy(),
            dll_destination.to_string_lossy()
        ));
    }
}

#[cfg(all(windows, target_env = "msvc"))]
fn find_cargo_target_dir() -> PathBuf {
    let pkg_name =
        env::var("CARGO_PKG_NAME").expect("Environment variable `CARGO_PKG_NAME` is missing.");

    let mut out_dir =
        PathBuf::from(env::var("OUT_DIR").expect("Environment variable `OUT_DIR` is missing."));

    loop {
        let target_directory = out_dir.file_name().unwrap();

        if target_directory.to_string_lossy().contains(&pkg_name) {
            break;
        } else if !out_dir.pop() {
            panic!("Unexpected build path: {}", out_dir.to_string_lossy());
        }
    }

    out_dir.pop();
    out_dir.pop();

    out_dir
}

#[cfg(any(unix, target_env = "gnu"))]
fn find_via_pkg_config(is_static: bool) -> bool {
    let mut pkg_config = pkg_config::Config::new();

    pkg_config.statik(is_static);

    pkg_config.probe("opus").is_ok()
}

/// Based on the OS or target environment we are building for,
/// this function will return an expected default library linking method.
///
/// If we build for Windows, MacOS, or Linux with musl, we will link statically.
/// However, if you build for Linux without musl, we will link dynamically.
///
/// **Info**:
/// This is a helper-function and may not be called if
/// if the `static`-feature is enabled, the environment variable
/// `LIBOPUS_STATIC` or `OPUS_STATIC` is set.
fn default_library_linking() -> bool {
    #[cfg(any(windows, target_os = "macos", target_env = "musl"))]
    {
        true
    }
    #[cfg(all(unix, target_env = "gnu"))]
    {
        false
    }
}

fn find_installed_opus() -> Option<String> {
    if let Ok(lib_directory) = env::var("LIBOPUS_LIB_DIR") {
        Some(lib_directory)
    } else if let Ok(lib_directory) = env::var("OPUS_LIB_DIR") {
        Some(lib_directory)
    } else {
        None
    }
}

fn is_static_build() -> bool {
    if cfg!(feature = "static") && cfg!(feature = "dynamic") {
        default_library_linking()
    } else if cfg!(feature = "static")
        || env::var("LIBOPUS_STATIC").is_ok()
        || env::var("OPUS_STATIC").is_ok()
    {
        true
    } else if cfg!(feature = "dynamic") {
        false
    } else {
        default_library_linking()
    }
}

fn main() {
    let installed_lib_directory = find_installed_opus();

    let is_static = is_static_build();

    #[cfg(any(unix, target_env = "gnu"))]
    {
        if find_via_pkg_config(is_static) {
            println!("cargo:info=Found `Opus` via `pkg_config`.");

            return;
        } else {
            println!("cargo:info=`pkg_config` could not find `Opus`.");
        }
    }

    let build_variable =
        std::env::var("OUT_DIR").expect("Environment variable `OUT_DIR` is missing.");

    let build_path = Path::new(&build_variable);

    build_opus(&build_path, is_static, &installed_lib_directory);
}
