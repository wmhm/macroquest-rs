//!

pub use tracing::{debug, error, info, trace, warn};

#[doc(hidden)]
#[cfg(feature = "logger")]
pub mod logger {
    pub use tracing::level_filters::LevelFilter;
    use tracing_appender::rolling::{RollingFileAppender, Rotation};
    use tracing_subscriber::prelude::*;

    use crate::mq;

    #[cfg(feature = "colors")]
    const SHOULD_COLOR: bool = true;

    #[cfg(not(feature = "colors"))]
    const SHOULD_COLOR: bool = false;

    pub fn init<F>(
        console_filter: Option<LevelFilter>,
        file_opts: Option<(LevelFilter, F)>,
    ) where
        F: Into<String>,
    {
        let console_layer = console_filter.map(|filter| {
            tracing_subscriber::fmt::layer()
                .with_writer(mq::console)
                .event_format(
                    tracing_subscriber::fmt::format()
                        .with_ansi(SHOULD_COLOR)
                        .without_time(),
                )
                .with_filter(filter)
        });

        let file_layer = file_opts.map(|(filter, filename)| {
            tracing_subscriber::fmt::layer()
                .with_writer(
                    RollingFileAppender::builder()
                        .rotation(Rotation::DAILY)
                        .filename_prefix(filename)
                        .filename_suffix("log")
                        .build(mq::paths().logs())
                        .expect("todo: error handling"),
                )
                .event_format(tracing_subscriber::fmt::format().with_ansi(false))
                .with_filter(filter)
        });

        tracing_subscriber::registry()
            .with(console_layer)
            .with(file_layer)
            .init();
    }
}
