use constime::comptime;

#[test]
fn test_base() {
	assert_eq!(comptime! { 239 + 259 * 23 }, 239 + 259 * 23);
	assert_eq!(comptime! { 2 + 259 * 7 }, 2 + 259 * 7);
	assert_eq!(comptime! { String::from("test") }, "test");
	assert_eq!(
		comptime! { format!("foo{}bar", String::from("/")) },
		"foo/bar"
	);
}

#[test]
fn test_dev_ureq() {
	let retrieved = comptime! {
		ureq::get("https://gist.githubusercontent.com/DvvCz/9972f1627f8418badb1736d6899d5f44/raw/b31a3627458a698dc029750dfc2572a6f8a131cf/test.txt").call().unwrap().into_string().unwrap()
	};

	assert_eq!(retrieved, "Hey there!");
}
