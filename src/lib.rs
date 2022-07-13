use std::{
	collections::{HashMap},
	path::PathBuf,
	str::FromStr,
};

use include_dir::{include_dir, Dir};
use pyo3::{types::PyModule, PyResult, Python};
use steamlocate::SteamDir;
use walkdir::WalkDir;

const LOCATORS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/locators");

pub struct SaveFinder {
	pub game_dirs: Vec<PathBuf>,
	pub save_dirs: Vec<(String, PathBuf, PathBuf)>,
	pub out_dir: PathBuf,
}

impl SaveFinder {
	pub fn new(out_dir: PathBuf) -> SaveFinder {
		SaveFinder {
			out_dir,
			game_dirs: vec![],
			save_dirs: vec![],
		}
	}

	pub fn find_games(&mut self, cache: &mut ApplicabilityCache) {
		let mut dirs = SteamDir::locate().unwrap();
		let apps = dirs
			.apps()
			.iter()
			.flat_map(|(_a, b)| b)
			.filter(|a| {
				let id = a.appid;
				if let Some(applicable) = cache.games.get(&id) {
					eprintln!("{}: {} (CACHED)", id, applicable);
					return *applicable;
				}
				let store_page =
					reqwest::blocking::get(format!("https://store.steampowered.com/app/{}", id))
						.map(|v| v.text().unwrap())
						.unwrap_or_else(|_| "".to_string());
				// If the store page contains steam cloud, we don't need to back it up
				let applicable = !store_page.contains(&"Steam Cloud".to_string());
				eprintln!("{}: {}", id, applicable);
				cache.games.insert(id, applicable);
				applicable
			})
			.map(|v| v.path.clone())
			.collect::<Vec<_>>();
		self.game_dirs = apps
	}

	pub fn find_saves(&mut self) {
		self.save_dirs = self
			.game_dirs
			.iter()
			.map(|path| {
				(
					path,
					path.iter().last().unwrap().to_string_lossy().to_string(),
				)
			})
			.map(|(game_path, game_name)| -> anyhow::Result<_> {
				let locator = LOCATORS
					.get_file(format!("{}.py", game_name))
					.unwrap_or_else(|| LOCATORS.get_file("Default.py").unwrap())
					.contents_utf8()
					.unwrap();
				let save_path = Python::with_gil(|py| -> PyResult<String> {
					let module = PyModule::from_code(py, locator, "locator.py", "Locator")?;
					Ok(module
						.call_method("locator", (game_path.to_str().unwrap(),), None)?
						.to_string())
				})?;
				Ok((
					game_name,
					game_path.clone(),
					PathBuf::from_str(&save_path).unwrap(),
				))
			})
			.inspect(|v| {
				if let Err(e) = v {
					println!("{:#?}", e)
				}
			})
			.flatten()
			.collect()
	}

	pub fn copy_saves(&self) {
		for (game_name, game_path, save_path) in &self.save_dirs {
			for file in WalkDir::new(save_path)
				.into_iter()
				.inspect(|v| {
					if v.is_err() {
						println!("{:#?}", v)
					}
				})
				.flatten()
			{
				let _result: anyhow::Result<()> = (|| {
					let file_path = file.path();
					let rel_path = file_path
						.strip_prefix(game_path.clone())
						.unwrap_or(file_path);
					let from_path = game_path.join(&rel_path);
					let to_path = self
						.out_dir
						.join(PathBuf::from_str(game_name)?)
						.join(&rel_path);
					let dir = to_path.ancestors().next().unwrap();
					std::fs::create_dir_all(dir)?;
					std::fs::copy(from_path, to_path)?;
					Ok(())
				})();
			}
		}
	}
}

// A map that caches whether a given game supports cloud saves.
// Stored ondisk in your xdg cache directory.
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct ApplicabilityCache {
	pub games: HashMap<u32, bool>,
}

impl ApplicabilityCache {
	pub fn load() -> ApplicabilityCache {
		let src_dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from_str("/tmp/").unwrap());
		let src_file = src_dir.join("save_finder_cache.json");
		serde_json::from_slice(&std::fs::read(src_file).unwrap_or_default()).unwrap_or_default()
	}
	pub fn put(&self) {
		let src_dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from_str("/tmp/").unwrap());
		let src_file = src_dir.join("save_finder_cache.json");
		let out = serde_json::to_string_pretty(self).unwrap();
		std::fs::write(src_file, out).unwrap();
	}
}