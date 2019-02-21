use rust_bitbar::{Line, Plugin, SubMenu};

fn main() {
    let mut pl = Plugin::new();
    let mut line = Line::new("first line");
    line.set_color("red").set_href("http://google.com");

    let mut sub_menu = SubMenu::new();
    sub_menu.add_line(line);

    let status_line = Line::new("🍺🍺🍺");
    pl.set_status_line(status_line).set_sub_menu(sub_menu);

    pl.render();
}
