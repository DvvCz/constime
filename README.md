<h1 align="center"> constime </h1>

<p align="center">
	Zig's <code>comptime</code> for Rust, with zero dependencies.
</p>

<div align="center">
	<a href="https://github.com/DvvCz/constime/actions">
		<img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/DvvCz/constime/ci.yml?label=build">
	</a>
	<a href="https://crates.io/crates/constime">
		<img alt="Crates.io Version" src="https://img.shields.io/crates/v/constime">
	</a>
	<a href="https://github.com/DvvCz/constime/releases/latest">
		<img alt="GitHub Release" src="https://img.shields.io/github/v/release/DvvCz/constime">
	</a>
</div>

## Usage

```bash
cargo add constime
```

Dependencies in `comptime!` can be stored in either `[dependencies]` or `[build-dependencies]`, and must be explicitly imported using `extern crate`.

You will also need a build.rs file in order to force `[build-dependencies]` to compile.

## Example

```rust
fn main() {
	use constime::comptime;

	// Let's use a pure-build time dependency
	println!("Here's a fact about the number 5: {}", comptime! {
		extern crate ureq;
		ureq::get("http://numbersapi.com/5/math").call().unwrap().into_string().unwrap()
	});

	// Standard library works fine too.
	println!(
		"Compiled {} seconds after unix epoch.",
		comptime! {
			std::time::SystemTime::now()
				.duration_since(std::time::UNIX_EPOCH)
				.expect("Time went backwards")
				.as_secs()
		}
	);
}
```