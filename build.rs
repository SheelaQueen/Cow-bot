use std::collections::HashSet;
use std::fs;

// #weloveduplicatingcode
pub trait ExpectError<T> {
	fn expect_error(self, msg: &str) -> T;
}

impl<T, E: std::fmt::Debug> ExpectError<T> for Result<T, E> {
	fn expect_error(self, msg: &str) -> T {
		self.expect(&format!("\x1b[31;1m[ERROR] {}\x1b[0m", msg))
	}
}

fn main() {
	let mut module_entries: HashSet<String> = HashSet::new();
	let mut function_entries = vec![];

	for entry in fs::read_dir("src/commands").expect_error("Failed to read src/commands/ directory") {
		if let Ok(entry) = entry {
			let path = entry.path();
			if path.extension().map_or(false, |ext| ext == "rs") {
				if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
					if filename == "mod" {
						continue;
					}

					if filename.ends_with("_command") {
						module_entries.insert(format!("mod {};", filename));
						if let Some(command_name) = filename.strip_suffix("_command") {
							function_entries.push(format!("{}::{}()", filename, command_name));
						}
					} else {
						module_entries.insert(format!("pub mod {};", filename));
					}
				}
			}
		}
	}

	let mod_content = format!(
		"{}\n\npub fn get_all_commands() -> Vec<poise::Command<crate::Data, crate::types::Error>> {{\n    vec![{}]\n}}",
		module_entries.into_iter().collect::<Vec<_>>().join("\n"),
		function_entries.join(", ")
	);

	fs::write("src/commands/mod.rs", mod_content).expect_error("Failed to write to mod.rs");
}
