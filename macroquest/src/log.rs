//! Utilities for handling logging.
//!
//! This integrates with the popular [tracing](https://crates.io/crates/tracing)
//! crate (which itself integrates with the also popular
//! [log](https://crates.io/crates/log)) crate, to implement simple, best
//! practices logging within a MacroQuest plugin.
//!
//! This crate always has the [`error`], [`warn`], [`info`], [`debug`], and
//! [`trace`] macros re-exported from the [tracing](https://crates.io/crates/tracing)
//! crate, however it is preferred to depend on tracing and use its definitions
//! directly.
//!
//! When the optional `logger` feature is enabled, then this module also has
//! support for creating a [tracing-subscriber](https://crates.io/crates/tracing-subscriber)
//! logging sink that integrates tracing with MacroQuest. It can emit logging
//! events to the MacroQuest console, to files in the MacroQuest log directory,
//! and to the "Debug Spew" log.
//!
//! # Examples
//!
//! Simple logging of messages (for more information, see the
//! [tracing](https://crates.io/crates/tracing) crate).
//!
//! ```
//! # use macroquest::eq::ChatColor;
//! # use macroquest::plugin::{Hooks, Plugin};
//! # macroquest::plugin::setup!(MQRustLogging);
//! # #[derive(Debug)]
//! # struct MQRustLogging {}
//! # impl Plugin for MQRustLogging {
//! #     fn new() -> Self {
//! #         MQRustLogging { }
//! #     }
//! # }
//! use macroquest::log::debug;
//!
//! #[macroquest::plugin::hooks]
//! impl Hooks for MQRustLogging {
//!     fn incoming_chat(&self, line: &str, color: ChatColor) -> bool {
//!         // This logs a "debug" message, with a captured parameter named
//!         // "line" that uses the debug output of the `line` variable.
//!         debug!(?line, "chat message received");
//!
//!         false
//!     }
//! }
//! ```
//!
//! Setup the logger to log "INFO" or higher messages to the console, with
//! "DEBUG" messages logged to a file with a particular name.
//!
//! ```
//! # use macroquest::log::{ConsoleLogger, FileLogger, Logger, LevelFilter};
//! # use macroquest::plugin::{Hooks, Plugin};
//! # macroquest::plugin::setup!(MQRustLogging);
//! # #[derive(Debug)]
//! # struct MQRustLogging {}
//! # impl Plugin for MQRustLogging {
//! #     fn new() -> Self {
//! #         MQRustLogging { }
//! #     }
//! # }
//! #[macroquest::plugin::hooks]
//! impl Hooks for MQRustLogging {
//!     fn initialize(&self) {
//!         Logger::builder()
//!             .console(ConsoleLogger::builder().level(LevelFilter::INFO).build())
//!             .file(
//!                 FileLogger::builder()
//!                     .filename("MQRustLogging")
//!                     .level(LevelFilter::DEBUG)
//!                     .build(),
//!             )
//!             .build()
//!             .install();
//!     }
//! }
//! ```

pub use tracing::{debug, error, info, trace, warn};

#[cfg_attr(docsrs, doc(cfg(feature = "logger")))]
#[cfg(feature = "logger")]
mod logger {
    pub use tracing::level_filters::LevelFilter;
    use tracing_appender::rolling::{RollingFileAppender, Rotation};
    use tracing_subscriber::prelude::*;
    use typed_builder::TypedBuilder;

    use crate::mq;

    /// Implements logging to the MacroQuest console.
    ///
    /// This will log all events to the MacroQuest console, using either the
    /// native ImGui console (``ctrl+\` ``) or through `MQ2Chat` or
    /// `MQ2ChatWnd`.
    #[allow(clippy::module_name_repetitions)]
    #[derive(TypedBuilder)]
    pub struct ConsoleLogger {
        /// The maximum level of event to log to the console.
        level: LevelFilter,
    }

    /// Implements logging to a rotating file.
    ///
    /// This will log all events to a file in the MacroQuest logs directory,
    /// using the given filename (as a prefix), and rotating on some set
    /// cadence.
    #[allow(clippy::module_name_repetitions)]
    #[derive(TypedBuilder)]
    pub struct FileLogger {
        /// The maximum level of event to log to the file.
        level: LevelFilter,

        /// The name of the filename (without any extensions) that this logger
        /// will use.
        #[builder(setter(into))]
        filename: String,
    }

    /// Implements logging to multiple locations using MacroQuest standard
    /// practices while integrating with the
    /// [tracing](https://crates.io/crates/tracing) ecosystem.
    #[derive(TypedBuilder)]
    pub struct Logger {
        /// The (optional) console logger to log events to.
        #[builder(setter(strip_option))]
        console: Option<ConsoleLogger>,

        /// The (optional) file logger to log events to.
        #[builder(setter(strip_option))]
        file: Option<FileLogger>,
    }

    impl Logger {
        /// Install this logger as our global logger, consuming it in the
        /// process.
        #[allow(clippy::missing_panics_doc)]
        pub fn install(self) {
            let console_layer = self.console.map(|console| {
                tracing_subscriber::fmt::layer()
                    .with_writer(mq::console)
                    .event_format(
                        tracing_subscriber::fmt::format()
                            .with_ansi(true)
                            .without_time(),
                    )
                    .with_filter(console.level)
            });

            let file_layer = self.file.map(|file| {
                tracing_subscriber::fmt::layer()
                    .with_writer(
                        RollingFileAppender::builder()
                            .rotation(Rotation::DAILY)
                            .filename_prefix(file.filename)
                            .filename_suffix("log")
                            .build(mq::paths().logs())
                            .expect("invalid file logger configuration"),
                    )
                    .event_format(tracing_subscriber::fmt::format().with_ansi(false))
                    .with_filter(file.level)
            });

            tracing_subscriber::registry()
                .with(console_layer)
                .with(file_layer)
                .init();
        }
    }
}

#[cfg(feature = "logger")]
pub use logger::{ConsoleLogger, FileLogger, LevelFilter, Logger};
