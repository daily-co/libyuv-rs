# libyuv-rs

Raw FFI bindings to libyuv libraries.

### Quick start

This crate is meant to be used as a git submodule so you should add the
following to your Cargo.toml:

```toml
libyuv = { path = "PATH_TO/libyuv-rs/" }
```

The [original project](https://github.com/mycrl/libyuv-rs) automatically
downloads precompiled static libraries from the github repository. However, in
this project we include the static libraries in the `binaries` folder.

### Building

The libyuv crate will automatically find the precompiled static library files in
the submodule (inside the `binaries` folder), otherwise you can also specify the
path where you have your binaries:

* `YUV_LIBRARY_PATH` - libyuv static library path.

#### Building libyuv static library

To build the static library follow the instruction at
https://chromium.googlesource.com/libyuv/libyuv/+/HEAD/docs/getting_started.md.

For Linux and macOS is something like:

```
mkdir libyuv
cd libyuv
gclient config --name src https://chromium.googlesource.com/libyuv/libyuv
gclient sync
make -f linux.mk
```

### License

This is a fork of https://github.com/mycrl/libyuv-rs

[MIT](./LICENSE) Copyright (c) 2022 Mr.Panda.
