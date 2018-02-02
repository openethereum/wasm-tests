use std::{fs, env};
use std::path::PathBuf;

fn main() {
	let args = env::args().collect::<Vec<_>>();
	if args.len() != 2 {
		println!("Usage: {} gen <test.rs>", args[0]);
		return;
	}

	let file_name = &args[1];

	let toml = r#"
[package]
name = "wasm-tests"
version = "0.1.0"
authors = ["NikVolf <nikvolf@gmail.com>"]

[dependencies]
pwasm-std = { git = "https://github.com/paritytech/pwasm-std" }
pwasm-ethereum = { git = "https://github.com/nikvolf/pwasm-ethereum" }
bigint = { version = "4", default-features = false }

[lib]
name = "$file_name"
path = "main.rs"
crate-type = ["cdylib"]

[replace]
"pwasm-std:0.3.2" = { git = "https://github.com/paritytech/pwasm-std" }
	"#;

	let target_toml = toml.replace("$file_name", file_name);

	let mut toml_path = PathBuf::from("target");
	toml_path.push("tests");
	toml_path.push(file_name);

	fs::create_dir_all(&toml_path).expect(&format!("failed to create \"{}\" directory", toml_path.to_string_lossy()));
	let target_dir = toml_path.clone();

	toml_path.push("Cargo.toml");

	let mut source_path = PathBuf::from("src");
	source_path.push(&format!("{}.rs", file_name));

	let mut target_main_path = PathBuf::from(target_dir.clone());
	target_main_path.push("main.rs");

	fs::copy(source_path.clone(), target_main_path.clone()).expect(
		&format!("failed to copy {} to {}", source_path.to_string_lossy(), target_main_path.to_string_lossy()));

	{
		let mut f = fs::File::create(toml_path.clone()).expect(&format!("failed to create \"{}\" file", toml_path.to_string_lossy()));
		::std::io::Write::write_all(&mut f, &target_toml.as_bytes()[..])
			.expect("Failed to write toml");
	}
}