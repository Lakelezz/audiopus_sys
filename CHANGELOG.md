# Change Log

An overview of changes:

## [0.1.3]

Fixes build-issues related to `pkg-config`.

## [0.1.2]

This release adds the ability to bypass `pkg-config`.

### **Added:**

* Ignore `pkg-config` when `LIBOPUS_NO_PKG` or `OPUS_NO_PKG` is set.
* Print the dynamic/static build cause via `cargo:info`.
* Add missing repository-link in `Cargo.toml`.

## [0.1.1]

### **Added:**

* Copy Opus' source to `OUT_DIR` before building to avoid modifying and generating files outside of `OUT_DIR`.

### **Fixed:**
* Convert Unix-relevant files' EOLs from `CRLF` to `LF` inside the `opus`-folder.
* Resolve unused import warnings when building with Unix.
