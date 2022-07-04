use std::path::PathBuf;

use structopt::StructOpt;

fn main() {
	let args = Args::from_args();
	let mut f = save_finder::SaveFinder::new(args.out_dir);
	f.find_games();
	eprintln!(
		"Found {} games to which the software applies.",
		f.game_dirs.len()
	);
	f.find_saves();
	eprintln!("Successfully found saves for {} games.", f.save_dirs.len());
	f.copy_saves();
	eprintln!("Done!")
}

#[derive(structopt::StructOpt)]
struct Args {
	#[structopt(parse(from_os_str))]
	out_dir: PathBuf,
}
