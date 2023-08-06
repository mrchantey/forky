use anyhow::*;
// use std::pin::Pin;
use sweet::*;

sweet! {
	test "panic block" {
		std::panic::set_hook(Box::new(|_| {}));
		let func = Box::pin(async { panic!("hello");});
		let result = unwrap_panic_async(func).await;
		expect(result).to_be_err_str("hello")?;
		let _ = std::panic::take_hook();
	}

	test "panic func"{
		std::panic::set_hook(Box::new(|_| {}));
		let func = || Box::pin(async { panic!("hello");});
		let result = unwrap_panic_async((func)()).await;
		expect(result).to_be_err_str("hello")?;
		let _ = std::panic::take_hook();
	}
	test "error"{
		let func = || Box::pin(async { Err(anyhow!("hello"))});
		let result = unwrap_panic_async((func)()).await;
		expect(result).to_be_err_str("hello")?;
	}
}
