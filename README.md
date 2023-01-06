# Constime

This is a tiny alternative to <https://github.com/nhynes/comptime-rs>, with no dependencies and <80 LOC.

## Usage
```bash
cargo add constime
```

Note that in order to use dependencies in `comptime!`, you must either:
* Have it as a normal dependency in `[dependencies]`.
* Have it as a build dependency in `[build-dependencies]`, alongside:
  * A `build.rs` file to make rust compiles the dependencies.
  * Explicitly importing the crate using `extern crate`.

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