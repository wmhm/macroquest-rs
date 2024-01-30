pub use tracing::{debug, error, info, trace, warn};

#[doc(hidden)]
#[cfg(feature = "logger")]
pub mod logger {
    pub use tracing::level_filters::LevelFilter;
    use tracing_subscriber::prelude::*;

    use crate::mq;

    #[cfg(feature = "colors")]
    const SHOULD_COLOR: bool = true;

    #[cfg(not(feature = "colors"))]
    const SHOULD_COLOR: bool = false;

    pub fn init(console_filter: Option<LevelFilter>) {
        let console_layer = console_filter.map(|filter| {
            tracing_subscriber::fmt::layer()
                .with_writer(mq::ConsoleWriter::new)
                .event_format(
                    tracing_subscriber::fmt::format()
                        .with_ansi(SHOULD_COLOR)
                        .without_time(),
                )
                .with_filter(filter)
        });

        tracing_subscriber::registry().with(console_layer).init();
    }
}
