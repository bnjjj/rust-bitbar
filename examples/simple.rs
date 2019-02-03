use rust_bitbar::{Line, Plugin, SubMenu};

fn main() {
    let mut pl = Plugin::new();
    let mut line = Line::new("first line".to_string());
    line.color("red".to_string())
        .href("http://google.com".to_string());

    let mut sub_menu = SubMenu::new();
    sub_menu.line(line);

    let status_line = Line::new(String::from("🍺🍺🍺"));
    pl.status_line(status_line).sub_menu(sub_menu);

    pl.render();
}
