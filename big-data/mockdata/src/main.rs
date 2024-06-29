use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;
use mockdata::gennum;
use mockdata::genstr;

/// Mock data
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Output file name.
    #[arg(short, long)]
    out: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate integer
    Gennum {
        /// Seperate mode. 1: line; 2: space;
        #[arg(short, long, default_value_t = 1)]
        seperate: u8,

        /// The number of random integer to generate
        #[arg(short, long)]
        count: usize,

        /// Number type. u8, u16, u32, u64, i8, i16, i32, i64
        #[arg(short, long, default_value = "i32")]
        numtype: String,
    },
    /// Generate string
    Genstr {
        /// Seperate mode. 1: line; 2: space;
        #[arg(short, long, default_value_t = 1)]
        seperate: u8,

        /// The number of random string to generate
        #[arg(short, long)]
        count: usize,

        /// The min length of random string
        #[arg(long, default_value_t = 1)]
        minlen: usize,

        /// The max length of random string
        #[arg(long, default_value_t = 10)]
        maxlen: usize,
    },
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    info!("args: {:?}", args);

    match &args.command {
        Some(Commands::Gennum {
            seperate,
            count,
            numtype,
        }) => {
            info!("Gennum seperate: {:?}, count: {:?}", seperate, count);
            let processor = gennum::GenNum::new(args.out, *seperate, *count, numtype.to_string());
            processor.generate()?;
        }
        Some(Commands::Genstr {
            seperate,
            count,
            minlen,
            maxlen,
        }) => {
            info!("Genstr seperate: {:?}, count: {:?}", seperate, count);
            let processor = genstr::GenStr::new(args.out, *seperate, *count, *minlen, *maxlen);
            processor.generate()?;
        }
        None => {}
    }

    Ok(())
}
