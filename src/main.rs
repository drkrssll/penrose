mod bindings;
mod layout;

use penrose::{
    core::{bindings::parse_keybindings_with_xmodmap, Config, WindowManager},
    extensions::hooks::add_ewmh_hooks,
    x11rb::RustConn,
    Result,
};
use penrose_ui::{bar::Position, core::TextStyle, status_bar};
use tracing_subscriber::{self, prelude::*};

const FONT: &str = "ProFontIIx Nerd Font";
const MAIN_BORDER: u32 = 0xff0000ff;
const ALT_BORDER: u32 = 0x00000000;
const TEXT_COLOR: u32 = 0xffffffff;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let config = add_ewmh_hooks(Config {
        focused_border: MAIN_BORDER.into(),
        normal_border: ALT_BORDER.into(),
        border_width: 2,
        default_layouts: layout::layouts(),
        ..Config::default()
    });

    let style = TextStyle {
        fg: TEXT_COLOR.into(),
        bg: Some(ALT_BORDER.into()),
        padding: (6, 4),
    };

    let bar = status_bar(
        layout::BAR_PX,
        FONT,
        10,
        style,
        MAIN_BORDER,
        ALT_BORDER,
        Position::Top,
    )
    .unwrap();

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(bindings::raw_key_bindings())?;

    let wm = bar.add_to(WindowManager::new(
        config,
        key_bindings,
        bindings::mouse_bindings(),
        conn,
    )?);

    wm.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_parse_correctly_with_xmodmap() {
        let res = parse_keybindings_with_xmodmap(bindings::raw_key_bindings());

        if let Err(e) = res {
            panic!("{e}");
        }
    }
}
