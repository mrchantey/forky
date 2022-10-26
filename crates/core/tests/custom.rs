use core::testing::*;
use core::*;

fn main() { run() }

test!("expect", || {
	expect(0).to_be(0);
	expect(0f32).to_be(0f32);
	expect("foo").to_be("foo");
	// expect(0).to_be(1);
	let my_str: &str = "Howdy";
	expect(my_str).to_be("Howdy");
	// let my_string: String = String::from("Partner");
	// expect(my_string).to_be("Partner".to_string());
});
test!("vec - int", || {
	let v = vec![0, 1, 2];
	let first = *v.first().or_default();
	let last = *v.last().or_default();
	expect(first).to_be(0);
	expect(last).to_be(2);
});
// test!("vec - string", || {
// 	let v = vec!["foo","bar","baz"];
// 	let first = v.first().clone().or_default();
// 	let last = v.last().or_default();
// 	expect(first).to_be("foo");
// 	// expect(last).to_be(2);
// });

test!("string", || {
	expect("foo/bar".split("/").count()).to_be(2);
	// expect("foo\\bar".split("\\").count()).to_be(2);

	// expect(my_string).to_be("Partner".to_string());
});


// fn quietly()-> Result<()>{
// 	let mut stdout = stdout();
// 	stdout.execute(cursor::SavePosition)?;
// 	stdout.execute(Print("hello\n"))?;
// 	stdout.execute(cursor::MoveUp(1))?;
// 	// stdout.execute(cursor::RestorePosition)?;
// 	stdout.execute(Print("world\n"))?;
// 	// stdout.into_
// 	// stdout.execute(cursor::MoveTo)?;
// 	let pos = cursor::position().unwrap_or_default();
// 	log!(pos.0 pos.1);
// 	// stdout.execute(terminal::Clear(terminal::ClearType::All))?;
// 	let pos1 = cursor::position().unwrap_or_default();
// 	log!(pos1.0 pos1.1);
// 	Ok(())
// }

test!("cursor", || {
});

test!("split", || {
	// for foo in splt{
	// 	if splt.peek().is_some(){

	// 	}
	// }
	// let len = splt.count();
	// let len = 2;
	// let others = splt.take(len - 1);
	// let lst = default(splt.last());
	// splt.next
	// expect(lst).to
	// if Some(lst) {
	// assert_eq!(lst,"bazs");
	// lst
	// expe
	// println!("here i am");
});
