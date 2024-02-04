//!

use std::borrow::Cow;
use std::io;
use std::path::Path;
use std::sync::OnceLock;

use cansi::{Color, Intensity};
use once_cell::sync::Lazy;
use parking_lot::Mutex;

use crate::eq::ChatColor;
use crate::ffi::mq as mqlib;

static PATHS: OnceLock<Paths> = OnceLock::new();

#[allow(missing_docs)]
pub struct Paths<'a> {
    root:        &'a Path,
    config:      &'a Path,
    ini:         &'a Path,
    macros:      &'a Path,
    logs:        &'a Path,
    crash_dumps: &'a Path,
    plugins:     &'a Path,
    resources:   &'a Path,
    everquest:   &'a Path,
}

#[allow(missing_docs)]
impl<'a> Paths<'a> {
    #[must_use]
    pub fn root(&self) -> &Path {
        self.root
    }

    #[must_use]
    pub fn config(&self) -> &Path {
        self.config
    }

    #[must_use]
    pub fn ini(&self) -> &Path {
        self.ini
    }

    #[must_use]
    pub fn macros(&self) -> &Path {
        self.macros
    }

    #[must_use]
    pub fn logs(&self) -> &Path {
        self.logs
    }

    #[must_use]
    pub fn crash_dumps(&self) -> &Path {
        self.crash_dumps
    }

    #[must_use]
    pub fn plugins(&self) -> &Path {
        self.plugins
    }

    #[must_use]
    pub fn resources(&self) -> &Path {
        self.resources
    }

    #[must_use]
    pub fn everquest(&self) -> &Path {
        self.everquest
    }
}

#[allow(missing_docs)]
pub fn paths() -> &'static Paths<'static> {
    PATHS.get_or_init(|| Paths {
        root:        Path::new(mqlib::get_path_MQRoot()),
        config:      Path::new(mqlib::get_path_Config()),
        ini:         Path::new(mqlib::get_path_MQini()),
        macros:      Path::new(mqlib::get_path_Macros()),
        logs:        Path::new(mqlib::get_path_Logs()),
        crash_dumps: Path::new(mqlib::get_path_CrashDumps()),
        plugins:     Path::new(mqlib::get_path_Plugins()),
        resources:   Path::new(mqlib::get_path_Resources()),
        everquest:   Path::new(mqlib::get_path_EverQuest()),
    })
}

/// Write a line of text into the MacroQuest console
///
/// This text will show up in the MacroQuest console (`ctrl \`), or in MQ2Chat
/// or MQ2ChatWnd depending on which plugins you have loaded. It supports any of
/// the MacroQuest colors (see
/// [Color Codes](https://docs.macroquest.org/reference/commands/echo/?h=#color-codes))
/// or any of the standard 8 ANSI color codes (as well as the "Faint" intensity
/// modifier for dimmed or darker text).
///
/// This will use the the default [`ChatColor`], if you want to set a specific
/// [`ChatColor`], see [`write_chat_color`].
pub fn write_chat<'a, S>(line: S)
where
    S: Into<Cow<'a, str>>,
{
    write_chat_color(line, ChatColor::default());
}

/// Write a line of text into the MacroQuest console
///
/// This text will show up in the MacroQuest console (`ctrl \`), or in MQ2Chat
/// or MQ2ChatWnd depending on which plugins you have loaded. It supports any of
/// the MacroQuest colors (see
/// [Color Codes](https://docs.macroquest.org/reference/commands/echo/?h=#color-codes))
/// or any of the standard 8 ANSI color codes (as well as the "Faint" intensity
/// modifier for dimmed or darker text).
///
/// You must specify which [`ChatColor`] the line of text should use, if you
/// want to just use the default, see [`write_chat`].
pub fn write_chat_color<'a, S>(line: S, color: ChatColor)
where
    S: Into<Cow<'a, str>>,
{
    mqlib::write_chat_color(&colorize_line(line.into()), color.into());
}

/// Convert the standard 8 ANSI color codes into MacroQuest color codes
///
/// While MacroQuest has it's own color codes, the ANSI codes are far more
/// standard and will have crates already available to make working with them
/// easy.
fn colorize_line<'a, S>(line: S) -> Cow<'a, str>
where
    S: Into<Cow<'a, str>>,
{
    let line = line.into();
    match memchr::memchr(b'\x1b', line.as_bytes()) {
        Some(_) => {
            cansi::v3::categorise_text(&line)
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
                            // standard 8, adding Purple and Orange, so we'll only
                            // map the 8 standard ANSI codes.
                            match fg {
                                Color::Black | Color::BrightBlack => "b",
                                Color::Green | Color::BrightGreen => "g",
                                Color::Magenta | Color::BrightMagenta => "p",
                                Color::Red | Color::BrightRed => "r",
                                Color::Cyan | Color::BrightCyan => "t",
                                Color::Blue | Color::BrightBlue => "u",
                                Color::White | Color::BrightWhite => "w",
                                Color::Yellow | Color::BrightYellow => "o",
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
        None => line,
    }
}

trait ChatWriter {
    fn write_chat<'a, S>(&self, line: S)
    where
        S: Into<Cow<'a, str>>;
}

struct MacroQuestChatWriter;

impl ChatWriter for MacroQuestChatWriter {
    fn write_chat<'a, S>(&self, line: S)
    where
        S: Into<Cow<'a, str>>,
    {
        write_chat(line);
    }
}

struct InternalConsoleWriter<W: ChatWriter> {
    writer: W,
    buffer: Vec<u8>,
}

impl<W: ChatWriter> InternalConsoleWriter<W> {
    #[must_use]
    fn new(writer: W) -> Self {
        InternalConsoleWriter {
            writer,
            buffer: Vec::new(),
        }
    }
}

impl<W: ChatWriter> io::Write for InternalConsoleWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Validate that our incoming buffer is valid utf8, but we don't need to
        // actually store this data anywhere, because we're only using this to
        // validate.
        std::str::from_utf8(buf)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;

        // Add our incoming data to the end of our buffer.
        self.buffer.extend_from_slice(buf);

        // Iterate over newline positions in our buffer, printing out each new
        // line using `write_chat`.
        let mut written = 0;
        for pos in memchr::memchr_iter(b'\n', &self.buffer) {
            let line = &self.buffer[written..pos];
            written = pos + 1;

            // We need to turn our bytes into a &str so we can pass them into
            // write_chat. It should not be possible for this to _not_ be valid
            // utf8, as we've only added valid utf8 to our buffer.
            let line = std::str::from_utf8(line).expect("invalid utf8 in buffer");

            // Actually write our line of chat out.
            self.writer.write_chat(line);
        }

        // Remove our written bytes from our buffer
        self.buffer.drain(..written);

        // Return how many bytes we've written
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

static CONSOLE: Lazy<Mutex<InternalConsoleWriter<MacroQuestChatWriter>>> =
    Lazy::new(|| Mutex::new(InternalConsoleWriter::new(MacroQuestChatWriter)));

/// A handle to the global console stream of the current MacroQuest process.
///
/// Each handle shares a global buffer of data to be written to the standard
/// output stream.
///
/// Created by the [`console`] method.
///
/// # Note
///
/// The MacroQuest console *only* supports emitting whole lines at a time, and
/// as such you must write at least whole lines to the console or the console
/// writer will buffer until it receives a new line character.
pub struct Console {}

impl io::Write for Console {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        CONSOLE.lock().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        CONSOLE.lock().flush()
    }
}

/// Constructs a new handle to the console stream of the current MacroQuest
/// process.
///
/// Each handle returned is a reference to a shared, global buffer whose access
/// is synchronized via a mutex.
///
/// # Note
///
/// The MacroQuest console *only* supports emitting whole lines at a time, and
/// as such you must write at least whole lines to the console or the console
/// writer will buffer until it receives a new line character.
#[must_use]
pub fn console() -> Console {
    Console {}
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::io::Write;

    use colored::Colorize;

    use super::*;

    #[test]
    fn test_colorize_returns_borrowed_when_no_color() {
        assert!(matches!(
            colorize_line("this is a line with no formatting"),
            Cow::Borrowed(..)
        ));
    }

    #[test]
    fn test_colorize_returns_same_when_no_color() {
        assert_eq!(
            colorize_line("this is a line with no formatting"),
            "this is a line with no formatting"
        );
    }

    #[test]
    fn test_colorize_leaves_mq_colors_alone() {
        assert_eq!(
            colorize_line(
                "\x07b\x07-b\x07g\x07-g\x07m\x07-m\x07o\x07-o\x07p\x07-p\x07r\x07-r\x07t\x07-t\x07u\x07-u\x07w\x07-w\x07y\x07-ysome text\x07x"
            ),
            "\x07b\x07-b\x07g\x07-g\x07m\x07-m\x07o\x07-o\x07p\x07-p\x07r\x07-r\x07t\x07-t\x07u\x07-u\x07w\x07-w\x07y\x07-ysome text\x07x"
        );
    }

    #[test]
    fn test_colorize_converts_ansi() {
        use super::colorize_line as c;

        assert_eq!(c("black".black().to_string()), "\x07bblack\x07x");
        assert_eq!(c("black".black().dimmed().to_string()), "\x07-bblack\x07x");

        assert_eq!(c("green".green().to_string()), "\x07ggreen\x07x");
        assert_eq!(c("green".green().dimmed().to_string()), "\x07-ggreen\x07x");

        assert_eq!(c("magenta".magenta().to_string()), "\x07pmagenta\x07x");
        assert_eq!(
            c("magenta".magenta().dimmed().to_string()),
            "\x07-pmagenta\x07x"
        );

        assert_eq!(c("red".red().to_string()), "\x07rred\x07x");
        assert_eq!(c("red".red().dimmed().to_string()), "\x07-rred\x07x");

        assert_eq!(c("cyan".cyan().to_string()), "\x07tcyan\x07x");
        assert_eq!(c("cyan".cyan().dimmed().to_string()), "\x07-tcyan\x07x");

        assert_eq!(c("blue".blue().to_string()), "\x07ublue\x07x");
        assert_eq!(c("blue".blue().dimmed().to_string()), "\x07-ublue\x07x");

        assert_eq!(c("white".white().to_string()), "\x07wwhite\x07x");
        assert_eq!(c("white".white().dimmed().to_string()), "\x07-wwhite\x07x");

        assert_eq!(c("yellow".yellow().to_string()), "\x07oyellow\x07x");
        assert_eq!(
            c("yellow".yellow().dimmed().to_string()),
            "\x07-oyellow\x07x"
        );
    }

    struct TestChatWriter {
        lines: RefCell<Vec<String>>,
    }

    impl ChatWriter for TestChatWriter {
        fn write_chat<'a, S>(&self, line: S)
        where
            S: Into<Cow<'a, str>>,
        {
            let mut lines = self.lines.borrow_mut();
            lines.push(line.into().to_string());
        }
    }

    #[allow(clippy::unused_io_amount)]
    #[test]
    fn test_console_writer_writes_line() {
        let mut console = InternalConsoleWriter {
            writer: TestChatWriter {
                lines: RefCell::new(Vec::new()),
            },
            buffer: Vec::new(),
        };

        console
            .write_all(b"this is one line\nand this is another\n")
            .unwrap();

        console.write(b"this is a line without a new line").unwrap();

        assert_eq!(
            *console.writer.lines.borrow(),
            &["this is one line", "and this is another"]
        );

        console.write(b"\n").unwrap();

        assert_eq!(
            *console.writer.lines.borrow(),
            &[
                "this is one line",
                "and this is another",
                "this is a line without a new line"
            ]
        );
    }
}
