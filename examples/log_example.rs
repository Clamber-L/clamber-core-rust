use clamber_core::logger_start;
use tracing::info;

fn main() {
    let _guards = logger_start("example", None);

    info!("Hello, world!");
}
