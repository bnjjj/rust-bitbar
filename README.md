# rust-bitbar [![Travis Badge]][Travis] [![Version](https://img.shields.io/crates/v/rust-bitbar.svg)](https://crates.io/crates/rust-bitbar) [![Documentation](https://docs.rs/rust-bitbar/badge.svg)](https://docs.rs/rust-bitbar)


This repo contains a crate for creating [BitBar](https://github.com/matryer/bitbar) plugins in Rust.

## Usage

Here is a simple example : 

```rust
use rust_bitbar::{Line, Plugin, SubMenu};

fn main() {
    let mut pl = Plugin::new();
    let mut line = Line::new("first line".to_string());
    line.set_color("red".to_string())
        .set_href("http://google.com".to_string());

    let mut sub_menu = SubMenu::new();
    sub_menu.add_line(line);

    let status_line = Line::new(String::from("🍺🍺🍺"));
    pl.set_status_line(status_line).set_sub_menu(sub_menu);

    pl.render();
}
```

To have more details please check the [official documentation](https://docs.rs/rust-bitbar).

[Travis]: https://travis-ci.org/bnjjj/rust-bitbar
[Travis Badge]: https://travis-ci.org/bnjjj/rust-bitbar.svg?branch=master