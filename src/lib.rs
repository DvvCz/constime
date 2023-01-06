extern crate proc_macro;
use proc_macro::TokenStream;
use std::hash::{Hash, Hasher};

#[proc_macro]
pub fn comptime(code: TokenStream) -> TokenStream {
	let mut args = std::env::args();
	let Some(out_dir) = args
		.position(|arg| arg == "--out-dir")
		.and_then(|_| args.next())
	else {
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