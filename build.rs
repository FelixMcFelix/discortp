use std::{env, path::Path};

// heavily based upon https://raw.githubusercontent.com/libpnet/libpnet/master/pnet_packet/build.rs
fn main() {
	let pattern = "./src/**/*.rs.in";
	for entry in glob::glob(pattern).expect("Pattern must be valid") {
		if let Ok(path) = entry {
			let out_dir = env::var_os("OUT_DIR").expect("Invalid OUT_DIR.");
			let file = Path::new(path.file_stem().expect("Invalid file_stem."));
			let dst = Path::new(&out_dir).join(file);
			let mut registry = syntex::Registry::new();
			pnet_macros::register(&mut registry);
			registry.expand("", path, &dst).unwrap();
		}
	}
}
