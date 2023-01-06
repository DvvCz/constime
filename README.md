# Constime

This is a tiny alternative to <https://github.com/nhynes/comptime-rs>, with no dependencies and <80 LOC.

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