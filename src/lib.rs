/// # Example
///
/// ```
/// use rust_bitbar::{Line, Plugin, SubMenu};
///
/// let mut pl = Plugin::new();
/// let mut line = Line::new("first line");
/// line.set_color("red")
///     .set_href("http://google.com");
///
/// let mut sub_menu = SubMenu::new();
/// sub_menu.add_line(line);
///
/// let status_line = Line::new("🍺🍺🍺");
/// pl.set_status_line(status_line).set_sub_menu(sub_menu);
///
/// pl.render();
/// ```

/// New returns an empty Bitbar menu without any context
#[derive(Default)]
pub struct Plugin {
    pub status_bar: StatusBar,
    pub sub_menu: Option<SubMenu>,
}

/// Line holds the content, styling and behaviour of a line in a Bitbar
/// menu, both in the menu and submenus
#[derive(Default, Debug, Clone)]
pub struct Line {
    text: String,
    href: String,
    color: String,
    font: String,
    size: i64,
    terminal: bool,
    refresh: bool,
    drop_down: bool,
    length: i64,
    trim: bool,
    alternate: bool,
    emojize: bool,
    ansi: bool,
    bash: String,
    params: Vec<String>,
    template_image: String,
    image: String,
    hr: bool,
}

/// Style wraps options related to text presentation which can be added to a line
#[derive(Default, Debug)]
pub struct Style {
    pub color: String,
    pub font: String,
    pub size: i64,
    pub length: i64,
    pub trim: bool,
    pub emojize: bool,
    pub ansi: bool,
}

/// Cmd wraps options related to commands which can be added to a line using the
#[derive(Default, Debug)]
pub struct Cmd {
    pub bash: String,
    pub params: Vec<String>,
    pub terminal: bool,
    pub refresh: bool,
}

/// StatusBar holds one of more Lines of text which are rendered in the status bar.
/// Multiple Lines will be cycled through over and over
#[derive(Default, Debug)]
pub struct StatusBar {
    pub lines: Vec<Line>,
}

/// SubMenu contains a slice of SubMenuItems which can be Lines or additional
/// SubMenus. The Level indicates how nested the submenu is which is used during
/// render to prepend the correct number of `--` prefixes.
pub struct SubMenu {
    pub level: i64,
    pub lines: Vec<SubMenuItem>,
}

/// Enum which represent a sub menu item
pub enum SubMenuItem {
    Line(Line),
    SubMenu(Box<SubMenu>),
}

impl Plugin {
    /// Function to create empty plugin
    pub fn new() -> Self {
        Plugin::default()
    }

    pub fn set_status_line(&mut self, line: Line) -> &mut Self {
        self.status_bar.lines.push(line);
        self
    }

    pub fn set_sub_menu(&mut self, sub_menu: SubMenu) -> &mut Self {
        self.sub_menu = Some(sub_menu);
        self
    }

    pub fn render(&self) {
        print!("{}", self.to_string());
    }
}

impl std::string::ToString for Plugin {
    fn to_string(&self) -> String {
        let mut output = String::from("");
        for line in self.status_bar.lines.iter().as_ref() {
            let line_str = line.to_string();
            output = format!("{}{}\n", output, line_str);
        }
        output = format!("{}---\n", output);
        if self.sub_menu.is_some() {
            let curr_sm = self.sub_menu.as_ref().unwrap();
            output = format!("{}{}", output, render_sub_menu(curr_sm));
        }

        return output;
    }
}

impl SubMenu {
    /// Function to create empty sub menu
    pub fn new() -> Self {
        SubMenu {
            level: 0,
            lines: vec![],
        }
    }
    /// Line creates a line adding text to the dropdown which will be added after
    /// the main dropdown delimiter (`---`).
    pub fn add_line(&mut self, line: Line) -> &mut Self {
        self.lines.push(SubMenuItem::Line(line));
        self
    }

    /// NewSubMenu creates a nested submenu off a submenu.
    pub fn add_sub_menu(&mut self, sub_menu: SubMenu) -> &mut Self {
        self.lines.push(SubMenuItem::SubMenu(Box::new(sub_menu)));
        self
    }

    /// HR turns a line into a horizontal delimiter, useful for breaking menu items
    /// into logical groups.
    pub fn add_hr(&mut self) -> &mut Self {
        let line = Line {
            text: "---".to_string(),
            hr: true,
            ..Default::default()
        };

        self.lines.push(SubMenuItem::Line(line));
        self
    }
}

impl Line {
    /// Function to create empty line
    pub fn new<T: Into<String>>(text: T) -> Self {
        Line {
            text: text.into(),
            ..Default::default()
        }
    }
    /// Change text of the line
    pub fn set_text<T: Into<String>>(&mut self, text: T) -> &mut Self {
        self.text = text.into();
        self
    }

    /// Style provides a alternate method for setting the text style related options.
    pub fn set_style(&mut self, style: Style) -> &mut Self {
        self.color = style.color;
        self.font = style.font;
        self.size = style.size;
        self.length = style.length;
        self.trim = style.trim;
        self.emojize = style.emojize;
        self.ansi = style.ansi;
        self
    }

    /// command provides a alternate method for setting the bash script and
    /// params along with some related flags via a Cmd struct.
    pub fn set_command(&mut self, cmd: Cmd) -> &mut Self {
        self.bash = cmd.bash;
        self.params = cmd.params;
        self.terminal = cmd.terminal;
        self.refresh = cmd.refresh;
        self
    }

    /// href adds a URL to the line and makes it clickable.
    pub fn set_href<T: Into<String>>(&mut self, href: T) -> &mut Self {
        self.href = href.into();
        self
    }
    /// Color sets the lines font color, can take a name or hex value.
    pub fn set_color<T: Into<String>>(&mut self, color: T) -> &mut Self {
        self.color = color.into();
        self
    }

    /// Font sets the lines font.
    pub fn set_font<T: Into<String>>(&mut self, font: T) -> &mut Self {
        self.font = font.into();
        self
    }

    /// Size sets the lines font size.
    pub fn set_size(&mut self, size: i64) -> &mut Self {
        self.size = size;
        self
    }

    /// Bash makes makes the line clickable and adds a script that will be run on click.
    pub fn set_bash<T: Into<String>>(&mut self, bash: T) -> &mut Self {
        self.bash = bash.into();
        self
    }
    /// Params adds arguments which are passed to the script specified by line.bash()
    pub fn set_params(&mut self, params: Vec<String>) -> &mut Self {
        self.params = params;
        self
    }

    /// Terminal sets a flag which controls whether a Terminal is opened when the bash
    /// script is run.
    pub fn set_terminal(&mut self, terminal: bool) -> &mut Self {
        self.terminal = terminal;
        self
    }

    /// Refresh controls whether clicking the line results in the plugin being refreshed.
    /// If the line has a bash script attached then the plugin is refreshed after the
    /// script finishes.
    pub fn set_refresh(&mut self, refresh: bool) -> &mut Self {
        self.refresh = refresh;
        self
    }

    /// DropDown sets a flag which controls whether the line only appears and cycles in the
    /// status bar but not in the dropdown.
    pub fn set_drop_down(&mut self, drop_down: bool) -> &mut Self {
        self.drop_down = drop_down;
        self
    }

    /// Length truncates the line after the specified number of characters. An elipsis will
    /// be added to any truncated strings, as well as a tooltip displaying the full string.
    pub fn set_length(&mut self, length: i64) -> &mut Self {
        self.length = length;
        self
    }

    /// Trim sets a flag to control whether leading/trailing whitespace is trimmed from the
    /// title. Defaults to `true`.
    pub fn set_trim(&mut self, trim: bool) -> &mut Self {
        self.trim = trim;
        self
    }

    /// Alternate sets a flag to mark a line as an alternate to the previous one for when the
    /// Option key is pressed in the dropdown.
    pub fn set_alternate(&mut self, alternate: bool) -> &mut Self {
        self.alternate = alternate;
        self
    }

    /// Emojize sets a flag to control parsing of github style :mushroom: into 🍄.
    pub fn set_emojize(&mut self, emojize: bool) -> &mut Self {
        self.emojize = emojize;
        self
    }

    /// Ansi sets a flag to control parsing of ANSI codes.
    pub fn set_ansi(&mut self, ansi: bool) -> &mut Self {
        self.ansi = ansi;
        self
    }
}

impl std::string::ToString for Line {
    fn to_string(&self) -> String {
        let mut res = vec![self.text.to_string()];
        let mut options: Vec<String> = vec!["|".to_string()];
        options.append(&mut render_style_options(self));
        options.append(&mut render_misc_options(self));
        options.append(&mut render_command_options(self));

        if options.len() > 1 {
            res.append(&mut options);
        }

        return res.join(" ");
    }
}

impl Style {
    /// Function to create empty style
    pub fn new() -> Self {
        Style::default()
    }
}

fn render_sub_menu(sub_menu: &SubMenu) -> String {
    let mut output = String::new();
    let mut prefix = String::new();

    if sub_menu.level > 0 {
        prefix = format!("{} ", "--".repeat(sub_menu.level as usize))
    }
    for line in sub_menu.lines.iter().as_ref() {
        match line {
            SubMenuItem::Line(current_line) => {
                if current_line.hr {
                    output = format!("{}{}{}\n", output, prefix.trim(), current_line.to_string());
                } else {
                    output = format!("{}{}{}\n", output, prefix, current_line.to_string());
                }
            }
            SubMenuItem::SubMenu(current_sub_m) => {
                output = format!("{}{}", output, render_sub_menu(&current_sub_m))
            }
        }
    }

    output
}

fn render_misc_options(line: &Line) -> Vec<String> {
    let mut misc_opts = vec![];

    if line.href != "" {
        misc_opts.push(format!("href='{}'", line.href));
    }
    if line.drop_down {
        misc_opts.push(format!("dropdown='{}'", line.drop_down));
    }
    if line.alternate {
        misc_opts.push(format!("alternate='{}'", line.alternate));
    }

    misc_opts
}

fn render_style_options(line: &Line) -> Vec<String> {
    let mut style_opts = vec![];
    if line.color != "" {
        style_opts.push(format!(r#"color="{}""#, line.color));
    }
    if line.font != "" {
        style_opts.push(format!(r#"font="{}""#, line.font));
    }
    if line.size > 0 {
        style_opts.push(format!("size={}", line.size));
    }
    if line.length > 0 {
        style_opts.push(format!("length={}", line.length));
    }
    if line.trim {
        style_opts.push(format!("trim={}", line.trim));
    }
    if line.emojize {
        style_opts.push(format!("emojize={}", line.emojize));
    }
    if line.ansi {
        style_opts.push(format!("ansi={}", line.ansi));
    }

    style_opts
}

fn render_command_options(line: &Line) -> Vec<String> {
    let mut command_opts = vec![];
    if line.bash != "" {
        command_opts.push(format!(r#"bash="{}""#, line.bash));
    }

    if line.params.len() > 0 {
        for i in 0..line.params.len() {
            command_opts.push(format!("param{}={}", i, line.params[i]));
        }
    }
    if line.terminal {
        command_opts.push(format!("terminal={}", line.terminal));
    }
    if line.refresh {
        command_opts.push(format!("refresh={}", line.refresh));
    }

    command_opts
}

#[test]
fn test_render_command_options() {
    let mut line = Line::new("here is a test".to_string());
    line.set_bash("echo test".to_string())
        .set_params(vec!["params1".to_string(), "params2".to_string()])
        .set_refresh(true);
    let resp = render_command_options(&line);

    assert_eq!(resp[0], r#"bash="echo test""#.to_string());
    assert_eq!(resp[1], r#"param0=params1"#.to_string());
    assert_eq!(resp[2], r#"param1=params2"#.to_string());
    assert_eq!(resp[3], "refresh=true".to_string());
}

#[test]
fn test_line_to_string() {
    let mut line = Line::new("here is a test".to_string());
    line.set_bash("echo test".to_string())
        .set_color("red".to_string())
        .set_params(vec!["params1".to_string(), "params2".to_string()])
        .set_refresh(true);
    let resp = line.to_string();

    assert_eq!(resp, r#"here is a test | color="red" bash="echo test" param0=params1 param1=params2 refresh=true"#.to_string());
}
