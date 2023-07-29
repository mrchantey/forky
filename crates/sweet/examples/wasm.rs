#![feature(imported_main)]
use forky_web::wait_for_millis;
use leptos::*;
pub use sweet::*;

// #[sweet_test]
// fn passes() -> anyhow::Result<()> {
// 	expect(true).to_be_true()?;
// 	Ok(())
// }

test! {"pases",
wait_for_millis(3000).await;
	log!("woah!");
	expect(true).to_be_true()?;
}
// test! {"html",
// 	log!("woah!");
// 		mount_to_body(|cx| {
// 		view! {cx,
// 			<div>
// 			"hello world!"
// 			</div>
// 		}
// 	});
// }
// test! {"fails", skip,
// 	log!("woah!");
// 	expect(true).to_be_false()?;
// }

// #[sweet_test]
// fn fails() -> Result<()> {
// 	expect(true).to_be_true()?;
// 	Ok(())
// }


/*

async fn run_test_async(){}

fn run_test()->Result<()>{
	let fut = async move {
		run_test_async().await;
	};
	let result = //TODO
	return result;
}


*/
