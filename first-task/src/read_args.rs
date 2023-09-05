use clap::Parser;

// Game score calculator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // Target offset
    #[arg(short, long)]
    pub offset: i32,
}

pub fn get_args() -> Args {
    Args::parse()
}
