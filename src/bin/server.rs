use anyhow::{Context, Result};
use simple_logger::SimpleLogger;
use log::LevelFilter;

pub const PUSH_PORT: i32 = 2048;
fn main() -> Result<()> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().with_context(||"Creating logger.")?;

    let context = zmq::Context::new();
    let socket  = context.socket(zmq::PUSH).with_context(|| "Creating Socket")?;
    socket.bind(format!("tcp://*:{}", PUSH_PORT).as_str()).with_context(|| "Binding socket")?;

    log::info!("Server is ready at location:{}", PUSH_PORT);

    log::info!("Server is ready at localhost:{}", PUSH_PORT);

    loop {
        let wallet1 =
    }


    Ok(())
}