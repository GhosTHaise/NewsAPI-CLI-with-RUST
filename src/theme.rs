use crossterm::style::Color::{Yellow,Rgb};
/* AnsiValue,Green, */
use termimad::{MadSkin,StyledChar};

pub fn default() -> MadSkin {
    let mut skin = MadSkin::default();
    /* Configuration du skin */
    skin.bold.set_fg(Yellow);
    skin.italic.set_bg(Rgb { r: 28, g: 28, b: 28 });

    skin.bullet = StyledChar::from_fg_char(Yellow, '+');
    skin.set_headers_fg(Yellow);
    
    skin.quote_mark = StyledChar::from_fg_char(Yellow, '|');
    skin.quote_mark.set_fg(Rgb{ 
        r: 215,
        g: 255,
        b: 135
    });
    skin.inline_code.set_fg(Rgb{
        r: 215,
        g: 0,
        b: 200,
    });
    skin.italic.set_fg(Rgb{
        r: 215,
        g: 255,
        b: 0,
    });

    skin
}