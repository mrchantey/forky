use extend::ext;
use proc_macro2::TokenStream;
use quote::quote;

#[ext]
pub impl Vec<TokenStream> {
	fn collect_comma_punct(self) -> TokenStream {
		let mut out = TokenStream::new();
		for (i, item) in self.into_iter().enumerate() {
			if i != 0 {
				out.extend(quote! {,});
			}
			out.extend(item);
		}
		out
	}
}
