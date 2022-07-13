use std::path::PathBuf;

use save_finder::ApplicabilityCache;
use structopt::StructOpt;

fn main() {
	let args = Args::from_args();
	let mut cache = ApplicabilityCache::load();
	let mut f = save_finder::SaveFinder::new(args.out_dir);
	f.find_games(&mut cache);
	eprintln!(
		"Found {} games to which the software might apply.",
		f.game_dirs.len()
	);
	f.find_saves();
	eprintln!("Successfully found saves for {} games.", f.save_dirs.len());
	f.copy_saves();
	cache.put();
	eprintln!("Done!");
}

#[derive(structopt::StructOpt)]
struct Args {
	#[structopt(parse(from_os_str))]
	out_dir: PathBuf,
}
