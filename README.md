# rust-bitbar [![Travis Badge]][Travis] [![Version](https://img.shields.io/crates/v/rust-bitbar.svg)](https://crates.io/crates/rust-bitbar) [![Documentation](https://docs.rs/rust-bitbar/badge.svg)](https://docs.rs/rust-bitbar)


This repo contains a crate for creating [BitBar](https://github.com/matryer/bitbar) plugins in Rust.

## Usage

Here is a simple example : 

```rust
use rust_bitbar::{new_line, new_plugin, new_sub_menu};

fn main() {
    let mut pl = new_plugin();
    let mut sub_menu = new_sub_menu();
    let mut line = new_line("first line".to_string());

    line.color("red".to_string())
        .href("http://google.com".to_string());

    sub_menu.line(line);

    pl.status_line(String::from("🍺🍺🍺"))
        .sub_menu(sub_menu);

    pl.render();
}
```

To have more details please check the [official documentation](https://docs.rs/rust-bitbar).

[Travis]: https://travis-ci.org/bnjjj/rust-bitbar
[Travis Badge]: https://travis-ci.org/bnjjj/rust-bitbar.svg?branch=master