use std::io;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn init_logging(level: Level) {
    let subscriber = FmtSubscriber::builder()
        .with_writer(io::stderr)
        .with_max_level(level)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
