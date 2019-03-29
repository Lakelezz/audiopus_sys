# Change Log

An overview of changes:

## [0.1.1]

### **Added:**

* Copy Opus' source to `OUT_DIR` before building to avoid modifying and generating files outside of `OUT_DIR`.

### **Fixed:**
* Convert Unix-relevant files' EOLs from `CRLF` to `LF` inside the `opus`-folder.
* Resolve unused import warnings when building with Unix.
