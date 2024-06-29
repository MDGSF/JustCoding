use anyhow::Result;
use clap::Parser;
use log::info;
use sortbigfile::sortbigfile;

/// Sort big file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file name.
    #[arg(short, long)]
    input: String,

    /// Output file name.
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    info!("args: {:?}", args);

    let processor = sortbigfile::SortBigFile::new(&args.input, &args.output);
    processor.process()?;

    Ok(())
}
