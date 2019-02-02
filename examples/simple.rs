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
