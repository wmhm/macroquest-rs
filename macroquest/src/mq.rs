use std::borrow::Cow;
use std::io;
use std::marker::PhantomData;

use crate::eq::ChatColor;
use crate::ffi::mq as mqlib;

pub fn write_chat_color<'a, S>(line: S, color: ChatColor)
where
    S: Into<Cow<'a, str>>,
{
    match line.into() {
        Cow::Borrowed(s) => mqlib::write_chat_color(s, color.into()),
        Cow::Owned(s) => mqlib::write_chat_color(s.as_str(), color.into()),
    }
}

pub fn write_chat<'a, S>(line: S)
where
    S: Into<Cow<'a, str>>,
{
    write_chat_color(line, ChatColor::Default)
}

pub struct ConsoleWriter(PhantomData<()>);

impl ConsoleWriter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ConsoleWriter(PhantomData)
    }
}

// TOOD: What if we get a partial line? we probably need to buff until we
//       have a full line if so.
impl io::Write for ConsoleWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;

        if let Some(raw) = buf.split_inclusive(|c| *c == b'\n').nth(0) {
            let line = std::str::from_utf8(raw)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;

            write_chat(colorize(line.trim_end_matches('\n')));
            written += raw.len();
        }

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(feature = "colors")]
fn colorize(line: &str) -> String {
    use cansi::{Color, Intensity};

    cansi::v3::categorise_text(line)
        .iter()
        .flat_map(|m| {
            // We ignore most of the ANSI codes, because WriteChatColor
            // doesn't support them in any meaningful way, but we can
            // support foreground colors.
            match m.fg {
                None => ["", "", "", m.text, ""],
                Some(fg) => [
                    // Control character that signifies a color code
                    "\x07",
                    // If we have a "Faint" color, then we'll use the darker
                    // variant of our colors.
                    match m.intensity.unwrap_or(Intensity::Normal) {
                        Intensity::Faint => "-",
                        _ => "",
                    },
                    // Map the ANSI colors to the MacroQuest color codes.
                    //
                    // MacroQuest supports 10 color codes instead of the
                    // standard 8, adding Purple and Orange. To support
                    // these two colors we'll use a similiar color with the
                    // blink ANSI code set.
                    // TODO: Purple and Orange
                    match fg {
                        Color::Black | Color::BrightBlack => "b",
                        Color::Green | Color::BrightGreen => "g",
                        Color::Magenta | Color::BrightMagenta => match m.blink {
                            // Purple is blinking Magenta.
                            Some(true) => "p",
                            // Otherwise, regular Magenta.
                            _ => "m",
                        },
                        Color::Red | Color::BrightRed => "r",
                        Color::Cyan | Color::BrightCyan => "t",
                        Color::Blue | Color::BrightBlue => "u",
                        Color::White | Color::BrightWhite => "w",
                        Color::Yellow | Color::BrightYellow => match m.blink {
                            // Orangle is blinking Yellow.
                            Some(true) => "o",
                            // Otherwise, regular Yellow.
                            _ => "y",
                        },
                    },
                    // The actual text wrapped by this ANSI color code.
                    m.text,
                    // The "reset back to the normal color" code.
                    "\x07x",
                ],
            }
        })
        .collect()
}

#[cfg(not(feature = "colors"))]
fn colorize(line: &str) -> &str {
    line
}
