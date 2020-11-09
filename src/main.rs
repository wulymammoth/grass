use anyhow::{Context, Result};
use log::info;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            match writeln!(writer, "{}", line) {
                Ok(ok) => ok,
                Err(err) => panic!(err)
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("starting up grass");

    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

#[test]
fn find_a_match() {
    let mut writer = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut writer);
    // `b` prefix makes the string a byte string literal `&[u8]` instead of `&str`
    assert_eq!(writer, b"lorem ipsum\n");
}
