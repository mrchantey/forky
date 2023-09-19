// use eyre::bail;
use quote::quote;
use rstml::node::Node;
use rstml::node::NodeAttribute;
use sweet::*;


// struct MyStruct;

fn run() -> anyhow::Result<()> {
	let tokens = quote! { <hello world>"hi"</hello> };
	let nodes = rstml::parse2(tokens).unwrap();
	let Node::Element(element) = &nodes[0] else {
		panic!()
	};
	// let path = &element.open_tag.name;
	// let foo = quote!(#path);
	// println!("{:?}", foo.to_string());
	let NodeAttribute::Attribute(attribute) = &element.attributes()[0] else {
		panic!()
	};
	let Node::Text(text) = &element.children[0] else {
		panic!()
	};
	assert_eq!(element.name().to_string(), "hello");
	assert_eq!(attribute.key.to_string(), "world");
	assert_eq!(text.value_string(), "hi");
	Ok(())
}

sweet! {
	it "works" {
		run()?;
	}
}
