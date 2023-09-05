mod check_hash;
mod find_counter;
mod find_hashes;
mod hash;
mod number_iterator;
mod read_args;

use find_hashes::find_hashes;
use read_args::get_args;

fn main() {
    let args = get_args();

    let result = find_hashes(args.number, args.find);

    for f in result {
        println!("{f}")
    }
}
