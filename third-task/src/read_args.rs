use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // Count '0' simbols from end
    #[arg(short = 'N', long)]
    pub number: u8,
    // Count search heshes
    #[arg(short = 'F', long)]
    pub find: u64,
}

pub fn get_args() -> Args {
    Args::parse()
}
