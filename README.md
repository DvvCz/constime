# Constime

This is a tiny alternative to https://github.com/nhynes/comptime-rs, with no dependencies and <70 LOC.
Note that it does not support externs, however.

```rust
fn main() {
	use constime::comptime;

	println!(
		"Compiled {} seconds after unix epoch",
		comptime! {
			std::time::SystemTime::now()
				.duration_since(std::time::UNIX_EPOCH)
				.expect("Time went backwards")
				.as_secs()
		}
	);
}
```