use web_sys::{window, UrlSearchParams};

pub fn search_param(query: &str) -> Option<String> {
	let search = window().unwrap().location().search().unwrap();
	let params = UrlSearchParams::new_with_str(search.as_str()).unwrap();
	params.get(query)
}
