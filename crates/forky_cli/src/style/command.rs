use super::*;




pub fn style(){
	let stylesheet = get_stylesheet(input);
	let classes = get_classes(&stylesheet);
	classes_to_tokens(classes).into();
}