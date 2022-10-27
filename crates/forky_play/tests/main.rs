// use magpie::hello_plugin;

#[test]
fn it_works() {
	// let result = playground::basic::hello_plugin::add(1., 2.);
	// assert_eq!(result, 3.);
}
#[test]
fn do_thing() {
	let a = 3;
	let b = 90;
	let c: Box<i32> = Box::new(b);

	assert_eq!(a, *c, "expected {}, received {}", a, *c);
}
