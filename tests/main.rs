use constime::comptime;

#[test]
fn test_base() {
	assert_eq!(comptime! { 239 + 259 * 23 }, 239 + 259 * 23);
	assert_eq!(comptime! { 2 + 259 * 7 }, 2 + 259 * 7);
	assert_eq!(comptime! { String::from("test") }, "test");
	assert_eq!(comptime! { format!("foo{}bar", String::from("/")) }, "foo/bar");
}
