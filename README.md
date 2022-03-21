# cargo-info
Query crates.io for crates details

## Install

```bash
cargo install --git https://github.com/light4/cargo-info.git --force
```

## Usage

```bash
~ on  master via 🐍 v3.10.2 🕙 12:59:50 
❯ cargo info serde

Crate:            serde
Version:          1.0.136
Default features: ["std"]
Features:         ["alloc", "derive", "rc", "std", "unstable"]
Description:      A generic serialization/deserialization framework
Downloads:        83548710
Homepage:         https://serde.rs
Documentation:    https://docs.serde.rs/serde/
Repository:       https://github.com/serde-rs/serde
License:          MIT OR Apache-2.0
Keywords:         ["serde", "serialization", "no_std"]
Last updated:     2 months ago
Version history:  

  VERSION         RELEASED        DOWNLOADS       

  1.0.136         2 months ago    4249742
  1.0.135         2 months ago    446444
  1.0.134         2 months ago    326054
  1.0.133         2 months ago    4866159
  1.0.132         3 months ago    1095893

  ... use -VV to show all 222 versions

```

## Credit

Forked from [imp/cargo-info](https://gitlab.com/imp/cargo-info)
