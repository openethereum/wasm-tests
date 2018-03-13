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
pwasm-std = "0.5.0"
pwasm-ethereum = "0.1.0"
bigint = { version = "4", default-features = false }

[lib]
name = "$file_name"
path = "main.rs"
crate-type = ["cdylib"]
"#;

	let target_toml = toml.replace("$file_name", file_name);

	let mut crate_dir_path = PathBuf::from("target");
	crate_dir_path.push("tests");
	crate_dir_path.push(file_name);

	fs::create_dir_all(&crate_dir_path).expect(&format!("failed to create \"{}\" directory", crate_dir_path.to_string_lossy()));
	let target_dir = crate_dir_path.clone();

	let mut source_path = PathBuf::from("src");
	source_path.push(&format!("{}.rs", file_name));

	let mut target_main_path = PathBuf::from(target_dir.clone());
	target_main_path.push("main.rs");

	fs::copy(source_path.clone(), target_main_path.clone()).expect(
		&format!("failed to copy {} to {}", source_path.to_string_lossy(), target_main_path.to_string_lossy()));

	{
		let mut toml_path = crate_dir_path.clone();
		toml_path.push("Cargo.toml");
		let mut f = fs::File::create(toml_path.clone()).expect(&format!("failed to create \"{}\" file", toml_path.to_string_lossy()));
		::std::io::Write::write_all(&mut f, &target_toml.as_bytes()[..])
			.expect("Failed to write toml");
	}
}
