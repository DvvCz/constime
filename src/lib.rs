extern crate proc_macro;
use proc_macro::TokenStream;
use std::hash::{Hash, Hasher};

#[proc_macro]
#[doc = include_str!("../README.md")]
pub fn comptime(code: TokenStream) -> TokenStream {
	let (mut externs, mut out_dir, mut args) = (vec![], None, std::env::args());
	while let Some(arg) = args.next() {
		if arg == "--out-dir" {
			out_dir = args.next();
		} else if arg == "--extern" {
			externs.push("--extern".to_owned());
			externs.push(args.next().unwrap());
		}
	}

	let Some(out_dir) = out_dir else {
		return "compile_error!(\"Could not find output directory.\")".parse().unwrap()
	};

	let code = format!("fn main(){{ println!(\"{{:?}}\", {{ {code} }}) }}");
	let mut hash = std::collections::hash_map::DefaultHasher::new();
	code.hash(&mut hash);
	let hash = hash.finish();

	let output_file = format!("{out_dir}{}constime-{hash}.exe", std::path::MAIN_SEPARATOR);
	let out_path = std::path::Path::new(&output_file);
	if out_path.exists() {
		let ext = out_path.with_extension("err");
		if ext.exists() {
			return format!(
				"compile_error!(r#\"{}\"#)",
				std::fs::read_to_string(ext).expect("Error when compiling")
			)
			.parse()
			.unwrap();
		}
	} else {
		let input_file = format!("{out_dir}{}constime-{hash}.rs", std::path::MAIN_SEPARATOR);
		std::fs::write(&input_file, code).expect("Failed to write temporary file to output");

		let rustc = std::process::Command::new("rustc")
			.stderr(std::process::Stdio::piped())
			.args([&input_file, "-o", &output_file])
			.args(["-L", &out_dir])
			.output();

		match rustc {
			Err(why) => return format!("compile_error!(r#\"{}\"#)", why).parse().unwrap(),
			Ok(output) if !output.status.success() => {
				return format!(
					"compile_error!(r#\"{}\"#)",
					std::str::from_utf8(&output.stderr).unwrap()
				)
				.parse()
				.unwrap()
			}
			_ => (),
		}
	}

	let out = std::process::Command::new(&output_file)
		.stdout(std::process::Stdio::piped())
		.output();

	match out {
		Err(why) => {
			std::fs::write(out_path.with_extension("err"), why.to_string()).unwrap();
			format!("compile_error!(r#\"Failed to execute code: {why}\"#)")
				.parse()
				.unwrap()
		}
		Ok(out) => std::str::from_utf8(&out.stdout).unwrap().parse().unwrap(),
	}
}