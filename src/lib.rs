#![doc = include_str!("../README.md")]

extern crate proc_macro;
use proc_macro::TokenStream;

use std::hash::{BuildHasher, RandomState};
use std::io::Write;

/// Properly passes an error message to the compiler without crashing macro engines.
macro_rules! build_error {
	($($arg:tt)*) => {
		format!("compile_error!(r#\"{}\"#)", format!($($arg)*))
			.parse::<TokenStream>()
			.unwrap()
	};
}

#[proc_macro]
#[doc = include_str!("../README.md")]
pub fn comptime(code: TokenStream) -> TokenStream {
	let mut out_dir = None;
	let mut externs = vec![];

	let mut args = std::env::args();
	while let Some(arg) = args.next() {
		// Push deps to rustc so you don't need to explicitly link with 'extern crate'
		if arg == "--extern" {
			externs.push(args.next().unwrap());
		} else if arg == "--out-dir" {
			out_dir = args.next().map(std::path::PathBuf::from);
		}
	}

	if out_dir.is_none() {
		let out = std::env::current_dir().unwrap().join("target").join("debug").join("deps");
		if out.exists() {
			out_dir = Some(out);
		}
	}

	let Some(out_dir) = out_dir else {
		return build_error!("Could not find output directory.");
	};

	let wrapped_code = format!(r#"
		fn main() {{
			println!("{{:?}}", {{ {code} }});
		}}
	"#);

	let hash = RandomState::new().hash_one(&wrapped_code);

	let constime_base = out_dir.join("constime");
	if !constime_base.exists() {
		std::fs::create_dir(&constime_base).unwrap();
	}

	let evaluator_base = constime_base
		.join(hash.to_string());

	if !evaluator_base.exists() { // This hasn't been compiled yet.
		let mut rustc = std::process::Command::new("rustc");
		rustc
			.stderr(std::process::Stdio::piped())
			.stdin(std::process::Stdio::piped())
			.current_dir(constime_base)
			.arg("-L")
			.arg(out_dir)
			.arg("-o")
			.arg(&evaluator_base)
			.arg("-");

		for ext in &externs {
			rustc.arg("--extern").arg(ext);
		}

		let Ok(mut rustc) = rustc.spawn() else {
			return build_error!("Failed to spawn rustc");
		};

		// Avoid deadlock by containing stdin handling in its own scope
		if let Some(mut stdin) = rustc.stdin.take() {
			if stdin.write_all(wrapped_code.as_bytes()).is_err() {
				return build_error!("Failed to write to rustc stdin");
			};
		} else {
			return build_error!("Failed to open stdin for rustc");
		}

		let Ok(output) = rustc.wait_with_output() else {
			return build_error!("Failed to wait for rustc");
		};

		if !output.status.success() {
			return build_error!("{}", String::from_utf8_lossy(&output.stderr));
		}
	}

	let out = std::process::Command::new(&evaluator_base)
		.stdout(std::process::Stdio::piped())
		.output();

	match out {
		Err(why) => return build_error!("Failed to execute code: {why}"),
		Ok(out) => {
			let out = String::from_utf8_lossy(&out.stdout);

			let Ok(out) = out.parse() else {
				return build_error!("Failed to parse output into a TokenStream");
			};

			return out;
		}
	}
}
