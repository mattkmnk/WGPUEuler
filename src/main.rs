use abstrct::run;

use tracing::error;

fn main() {
    init_logger();

    match tokio::runtime::Runtime::new() {
        Ok(runtime) => runtime.block_on(run()),
        Err(err) => error!("Error creating tokio runtime: {}", err),
    }
}

fn init_logger() {
    // tracing_subscriber::fmt()
    //     .with_target(true)
    //     .with_max_level(tracing::Level::TRACE)
    //     .with_writer(std::io::stdout)
    //     .init();
    tracing_subscriber::fmt()
        .with_env_filter("abstrct=trace")
        .with_target(true)
        .init();
}
