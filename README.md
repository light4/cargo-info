# cargo-info
Query crates.io for crates details

[![CI](https://github.com/light4/cargo-info/actions/workflows/test.yaml/badge.svg)](https://github.com/light4/cargo-info/actions/workflows/test.yaml)
[![build-and-release](https://github.com/light4/cargo-info/actions/workflows/build-and-release.yaml/badge.svg)](https://github.com/light4/cargo-info/actions/workflows/build-and-release.yaml)

## Install

```bash
cargo install --git https://github.com/light4/cargo-info.git --force
# Archlinux aur https://aur.archlinux.org/packages/cargo-info
yay -S cargo-info
```

## Usage

```bash
~ on  master via 🐍 v3.10.2 🕙 12:59:50
❯ cargo info serde

Crate:            serde (https://crates.io/crates/serde)
Version:          1.0.136
Default features: ["std"]
Features:         ["alloc", "derive", "rc", "std", "unstable"]
Description:      A generic serialization/deserialization framework
Downloads:        83622745
Homepage:         https://serde.rs
Documentation:    https://docs.serde.rs/serde/
Repository:       https://github.com/serde-rs/serde
License:          MIT OR Apache-2.0
Keywords:         ["serde", "serialization", "no_std"]
Last updated:     2 months ago
Version history:

  VERSION         RELEASED        DOWNLOADS

  1.0.136         2 months ago    4297338
  1.0.135         2 months ago    446780
  1.0.134         2 months ago    326650
  1.0.133         2 months ago    4868915
  1.0.132         3 months ago    1097574

  ... use -VV to show all 222 versions

```

## Credit

Forked from [imp/cargo-info](https://gitlab.com/imp/cargo-info)
