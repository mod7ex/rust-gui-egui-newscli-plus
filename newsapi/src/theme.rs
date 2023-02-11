use termimad::{MadSkin, StyledChar, crossterm::style::Color::{Rgb, Yellow}};

pub fn default() -> MadSkin {
    let mut skin = MadSkin::default();

    skin.bold.set_fg(Yellow);

    skin.italic.set_bg(Rgb { r: 28, g: 28, b: 28 });

    skin.bullet = StyledChar::from_fg_char(Yellow, '⟡');

    skin.quote_mark = StyledChar::from_fg_char(Yellow, '▐');

    skin.set_headers_fg(Yellow);

    skin.quote_mark.set_fg(Rgb { r: 215, g: 255, b: 135 });

    skin.inline_code.set_fg(Rgb { r: 134, g: 233, b: 42 });

    skin.italic.set_fg(Rgb { r: 215, g: 255, b: 0 });  

    skin
}