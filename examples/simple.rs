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
