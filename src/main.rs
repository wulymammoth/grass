use anyhow::{Context, Result};
use log::info;
use std::{thread, time::Duration};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("starting up grass");

    // NOTE: https://rust-cli.github.io/book/in-depth/signals.html#first-off-handling-ctrlc
    // - in real-world applications, we may want to set an `Arc<AtomicBool>>` (boolean that is
    // shared between threads) in the signal handler, and in hot loops, or when waiting on a
    // thread, perform a periodic check of this value and break if it is ever true
    let signal_handler = move || println!("received Ctrl+C!");
    ctrlc::set_handler(signal_handler).expect("Error setting Ctrl-C handler");
    // this allow me to send a ctrl-c signal
    thread::sleep(Duration::from_secs(2));

    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;

    grass::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
