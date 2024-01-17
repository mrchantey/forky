use forky_core::*;
// use std::rc::Rc;
use std::sync::{
	Arc,
	Mutex,
};
use std::thread;
use sweet::*;

sweet! {

	it "spawns"{
		let handle = thread::spawn(|| {
			for _ in 1..10 {
					// println!("hi number {} from the spawned thread!", i);
					utility::sleep_ms(1);
				}
			});
			handle.join().unwrap();

			for _ in 1..5 {
				// println!("hi number {} from the main thread!", i);
				utility::sleep_ms(100);
			}
			// println!("all done!");
	}

	test "mutex"{
		let m = Mutex::new(5);
		{
			let mut num = m.lock().unwrap();
			*num = 6;
		}
		expect(*m.lock().unwrap()).to_be(6)?;
	}

	test "shared mutex"{
		let counter = Arc::new(Mutex::new(0));
		let mut handles = vec![];
		for _ in 0..10{
			let counter = Arc::clone(&counter);
			let handle = thread::spawn(move ||{
				let mut num = counter.lock().unwrap();

				*num += 1;
			});
			handles.push(handle);
		}
		for handle in handles{
			handle.join().unwrap();
		}
		let result = *counter.lock().unwrap();
		expect(result).to_be(10)?;
	}


	test "dyn mutex"{
		struct Foo{}
		trait Bar{
			fn get(&mut self)->i32;
		}
		impl Bar for Foo{
			fn get(&mut self)->i32{3}
		}

		fn do_something(a:Arc<Mutex<dyn Bar + Send>>){
			let a = Arc::clone(&a);
			let _closure = thread::spawn(move ||{
				let _foo = a.lock().unwrap();
			});
		}

		let a = Arc::new(Mutex::new(Foo{}));
		do_something(a);

	}
}
