use clap::Parser;

/// A program for finding numbers whose hash ends with a certain number of characters '0'
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Count '0' symbols from end
    #[arg(short = 'N', long)]
    pub number: u8,
    /// Count search hashes
    #[arg(short = 'F', long)]
    pub find: u64,
}

pub fn get_args() -> Args {
    Args::parse()
}
