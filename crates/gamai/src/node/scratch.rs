struct Node0<const DEPTH: usize>;


impl<const DEPTH: usize> Node0<DEPTH> {
	// const NEXT_DEPTH: usize = DEPTH + 1;
	pub fn child() -> Node0<{ DEPTH + 1 }>
	where
		[(); DEPTH + 1]:,
	{
		Node0::<{ DEPTH + 1 }>
	}
}

trait Node {
	fn into_child() -> impl Node;
}

// struct Foo<const DEPTH: usize>;

// impl<const DEPTH: usize> Node for Node0<DEPTH>
// where
// 	[(); DEPTH + 1]:,
// 	// 	Node0<{ DEPTH + 1 }>: Node,
// {
// 	fn into_child() -> impl Node
// 	where
// 		[(); DEPTH + 1]:,
// 		// [(); DEPTH + 1]:, // <Foo<{ DEPTH + 1 }> as Node>::DEPTH
// 	{
// 		Node0::<{ DEPTH + 1 }>
// 		// Node0::<DEPTH>::child()
// 		// Foo::<{ DEPTH + 1 }>
// 		// Foo::<{ DEPTH + 1 }>
// 	}
// }
