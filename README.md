# libyuv-rs

Raw FFI bindings to libyuv libraries.

## Quick start

This crate is meant to be used as a git submodule so you should add the
following to your Cargo.toml:

```toml
libyuv = { path = "PATH_TO/libyuv-rs/" }
```

The [original project](https://github.com/mycrl/libyuv-rs) automatically
downloads precompiled static libraries from the github repository. However, in
this project we include the static libraries in the `binaries` folder.

## Building

The libyuv crate will automatically find the precompiled static library files in
the submodule (inside the `binaries` folder), otherwise you can also specify the
path where you have your binaries:

* `YUV_LIBRARY_PATH` - libyuv static library path.

## Building libyuv static library

To build the static library first you need `depot_tools`:

```
git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git

export PATH=/PATH/To/depot_tools:$PATH
```

Then, clone libyuv repo:

```
mkdir libyuv
cd libyuv
gclient config --name src https://chromium.googlesource.com/libyuv/libyuv
gclient sync
```

The following sections show you how to build the static library `libyuv.a`. Once
that is built copy it to the `binaries` folder with the proper name:
`libyuv-{OS}-{ARCH}.a`.

### Linux (x86_64 and aarch64)

The following assumes the host machine is `x86_64`:

```
make V=1 -f linux.mk
```

To cross-compile to `aarch64`:

```
CC=aarch64-linux-gnu-gcc \
  CXX=aarch64-linux-gnu-g++ \
  AR=aarch64-linux-gnu-ar \
  make V=1 -f linux.mk
```

### macOS (x86_64 and aarch64)

The following assumes the host machine is `aarch64`:

```
make V=1 -f linux.mk
```

To cross-compile to `x86_64`:

```
CC="clang -arch x86_64" CXX="clang++ -arch x86_64" make V=1 -f linux.mk
```

### License

This is a fork of https://github.com/mycrl/libyuv-rs

[MIT](./LICENSE) Copyright (c) 2022 Mr.Panda.
