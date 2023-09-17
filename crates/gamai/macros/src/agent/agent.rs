use super::*;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::AttributeArgs;
use syn::ItemStruct;
use syn::Visibility;

pub struct Agent {
	pub num_choices: usize,
	pub ident: Ident,
	pub vis: Visibility,
	pub choice_params: TokenStream,
	pub choice_bounds: TokenStream,
	// pub fields: TokenStream,
	// pub fields_typed: TokenStream,
	pub builder: AgentBuilder,
}

impl Agent {
	pub fn new(item: ItemStruct, attr: AttributeArgs) -> Self {
		let num_choices = parse_attributes(attr).unwrap_or(2);
		let (choice_generic_params, choice_generic_bounds) =
			choice_generics(num_choices);
		// let (fields, fields_typed) = fields(num_choices);
		let builder = AgentBuilder::new(&item, num_choices);
		Self {
			builder,
			num_choices,
			choice_params: choice_generic_params,
			choice_bounds: choice_generic_bounds,
			// fields,
			// fields_typed,
			ident: item.ident,
			vis: item.vis,
		}
	}
	pub fn parse(
		attr: proc_macro::TokenStream,
		item: proc_macro::TokenStream,
	) -> proc_macro::TokenStream {
		let attr = syn::parse_macro_input!(attr as syn::AttributeArgs);
		let item = syn::parse_macro_input!(item as syn::ItemStruct);
		let agent = Agent::new(item, attr);

		let builder_impl = impl_builder(&agent);
		let self_impl = impl_self(&agent);
		let agent_impl = impl_agent(&agent);
		let sets_impl = impl_sets(&agent);
		let bundle_impl = impl_bundle(&agent);

		quote! {
			use bevy::prelude::*;
			use gamai::*;
			#self_impl
			#sets_impl
			#builder_impl
			#agent_impl
			#bundle_impl
		}
		.into()
	}
}

fn parse_attributes(attr: syn::AttributeArgs) -> Option<usize> {
	if let Some(first) = attr.first() {
		if let syn::NestedMeta::Lit(syn::Lit::Int(val)) = first {
			return val.base10_parse::<usize>().ok();
		}
	}
	return None;
}
