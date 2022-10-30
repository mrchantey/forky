/// All the root `Describe` blocks defined in the current `demonstrate!` instance
pub(crate) struct Root(pub(crate) Vec<Describe>);

impl Parse for Root {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut blocks = Vec::new();
        while !input.is_empty() {
            blocks.push(input.parse::<Describe>()?);
        }

        Ok(Root(blocks))
    }
}
