pub fn run_xr_test() {
	log!("WebXR - Starting...");
	set_panic_hook();
	let _ = init_and_run_xr(move |_time: f64, _frame: XrFrame| {
		log!("frame");
	});
	// let gl = create_webgl_context(true).unwrap();
	// let gl = Rc::new(gl);
	// let session = Rc::new(RefCell::new(None));
	// let result =
	// 	JsFuture::from(init_webxr(session.clone(), gl.clone())).await?;
	// run_xr(&session, );
	log!("WebXR - Initialized");
}
fn viewport_rect(view: &XrViewport) -> (i32, i32, i32, i32) {
	(view.x(), view.y(), view.width(), view.height())
}
