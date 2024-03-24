use js_sys::Array;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::Element;
use web_sys::HtmlCanvasElement;
use web_sys::ResizeObserver;
use web_sys::ResizeObserverEntry;
use web_sys::ResizeObserverSize;


/// Resize listener
/// When using with leptos, ensure it is moved to [`on_cleanup`] to avoid being dropped
pub struct ResizeListener {
	pub observer: ResizeObserver,
	pub cb: Closure<dyn FnMut(Array, ResizeObserver)>,
}

impl ResizeListener {
	pub fn new<F>(el: &Element, mut f: F) -> Self
	where
		F: FnMut(&ResizeObserverEntry) + 'static,
	{
		let cb = Closure::wrap(Box::new(
			move |entries: Array, _observer: ResizeObserver| {
				let entry = entries.get(0);
				let entry = entry.dyn_ref::<ResizeObserverEntry>().unwrap();
				f(entry);
			},
		) as Box<dyn FnMut(Array, ResizeObserver)>);
		let observer =
			ResizeObserver::new(&cb.as_ref().unchecked_ref()).unwrap();
		observer.observe(el);
		Self { cb, observer }
	}

	pub fn forget(self) { std::mem::forget(self); }
	/// utility function for parsing the entry, usually this is what you want
	/// https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/contentBoxSize
	pub fn parse_entry(entry: &ResizeObserverEntry) -> (u32, u32) {
		let first = entry.content_box_size().get(0);
		let first = first.unchecked_ref::<ResizeObserverSize>();
		let width = first.inline_size();
		let height = first.block_size();
		(width as u32, height as u32)
	}
	pub fn resize_canvas(canvas: HtmlCanvasElement) -> Self {
		let canvas2 = canvas.clone();
		Self::new(canvas.unchecked_ref(), move |entry| {
			let content_rect = entry.content_rect();
			canvas2.set_width(content_rect.width() as u32);
			canvas2.set_height(content_rect.height() as u32);
		})
	}
}

impl Drop for ResizeListener {
	fn drop(&mut self) { self.observer.disconnect(); }
}


#[cfg(feature = "leptos")]
#[allow(unused)]
pub use leptos_resize::*;
#[cfg(feature = "leptos")]
pub mod leptos_resize {
	use crate::prelude::*;
	use leptos::html::ElementDescriptor;
	use leptos::*;
	use web_sys::DomRect;
	use web_sys::ResizeObserverEntry;

	pub fn use_resize_listener<T: 'static + ElementDescriptor + Clone>(
		el: NodeRef<T>,
	) -> ReadSignal<Option<ResizeObserverEntry>> {
		let signal = create_rw_signal(None);
		let resize_listener = create_effect(move |_| {
			if let Some(container) = el.get() {
				let el = container.into_web_sys();
				let listener = ResizeListener::new(&el, move |entry| {
					signal.set(Some(entry.clone()));
				});
				Some(listener)
			} else {
				None
			}
		});

		on_cleanup(move || drop(resize_listener));

		signal.read_only()
	}


	/// Memoized version of [`use_resize_listener`], that also sets intial size based on [`get_bounding_client_rect`]
	pub fn use_size_listener<T: 'static + ElementDescriptor + Clone>(
		el: NodeRef<T>,
	) -> Signal<(u32, u32)> {
		let resize_listener = use_resize_listener(el);
		let func = create_memo(move |_| {
			// Default::default()
			let el = el();
			resize_listener()
				.map(|entry| ResizeListener::parse_entry(&entry))
				.unwrap_or_else(|| {
					el.map(|el| {
						let rect = el.into_web_sys().get_bounding_client_rect();
						(rect.x() as u32, rect.y() as u32)
					})
					.unwrap_or_default()
				})
		});
		func.into_signal()
	}

	/// Memoized version of [`use_resize_listener`], that also sets intial size based on [`get_bounding_client_rect`]
	pub fn use_size_listener_with_parent<
		T: 'static + ElementDescriptor + Clone,
	>(
		el: NodeRef<T>,
	) -> Signal<(u32, u32)> {
		let resize_listener = use_resize_listener(el);
		let func = create_memo(move |_| {
			// Default::default()
			let el = el();
			resize_listener()
				.map(|entry| ResizeListener::parse_entry(&entry))
				.unwrap_or_else(|| {
					el.map(|el| {
						let rect = el.into_web_sys().get_bounding_client_rect();
						(rect.x() as u32, rect.y() as u32)
					})
					.unwrap_or_default()
				})
		});
		func.into_signal()
	}



	/// calls [`Element::get_bounding_client_rect`] whenever this element or its parent is resized
	/// The resize calls are memoized
	pub fn use_dom_rect<T1: ElementDescriptor + Clone>(
		el: NodeRef<T1>,
	) -> Signal<DomRect> {
		let on_resize = use_size_listener(el);
		let func = move || {
			let _ = on_resize();
			el().map(|el| el.into_web_sys().get_bounding_client_rect())
				.unwrap_or_else(|| DomRect::new().unwrap())
		};
		func.into_signal()
	}
	pub fn use_dom_rect_with_parent<
		T1: ElementDescriptor + Clone,
		T2: ElementDescriptor + Clone,
	>(
		el: NodeRef<T1>,
		parent: NodeRef<T2>,
	) -> Signal<DomRect> {
		let on_parent_resize = use_size_listener(parent);
		let on_resize = use_size_listener(el);
		let func = move || {
			let _ = on_resize();
			let _ = on_parent_resize();
			el().map(|el| el.into_web_sys().get_bounding_client_rect())
				.unwrap_or_else(|| DomRect::new().unwrap())
		};
		func.into_signal()
	}
}




// pub fn sync_canvas_size(
// 	canvas: HtmlCanvasElement,
// ) -> FnClosure2<Array, ResizeObserver> {
// 	let canvas2 = canvas.clone();
// 	let cb = Closure::wrap(Box::new(
// 		move |entries: Array, _observer: ResizeObserver| {
// 			let entry = entries.get(0);
// 			let entry =
// 				entry.dyn_ref::<ResizeObserverEntry>().unwrap();
// 			// entry.
// 			let content_rect = entry.content_rect();
// 			canvas2.set_width(content_rect.width() as u32 - 1);
// 			canvas2.set_height(content_rect.height() as u32 - 1);
// 		},
// 	) as Box<dyn FnMut(Array, ResizeObserver)>);
// 	let observer = ResizeObserver::new(&cb.as_ref().unchecked_ref()).unwrap();
// 	observer.observe(canvas.unchecked_ref());
// 	cb
// }
