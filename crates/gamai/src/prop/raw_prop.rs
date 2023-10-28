use super::*;
use crate::*;
use bevy_ecs::prelude::*;

//lazily got gpt to generate instead of using macro

pub struct RawProp1<T1: IntoProp>(pub T1);

impl<T1: IntoProp> IntoPropBundle for RawProp1<T1> {
	fn into_bundle<Node: AiNode>(self) -> impl Bundle {
		Prop::<_, Node>::new(self)
	}
}
pub struct RawProp2<T1: IntoProp, T2: IntoProp>(pub T1, pub T2);

impl<T1: IntoProp, T2: IntoProp> IntoPropBundle for RawProp2<T1, T2> {
	fn into_bundle<Node: AiNode>(self) -> impl Bundle {
		(Prop::<_, Node>::new(self.0), Prop::<_, Node>::new(self.1))
	}
}

pub struct RawProp3<T1: IntoProp, T2: IntoProp, T3: IntoProp>(
	pub T1,
	pub T2,
	pub T3,
);

impl<T1: IntoProp, T2: IntoProp, T3: IntoProp> IntoPropBundle
	for RawProp3<T1, T2, T3>
{
	fn into_bundle<Node: AiNode>(self) -> impl Bundle {
		(
			Prop::<_, Node>::new(self.0),
			Prop::<_, Node>::new(self.1),
			Prop::<_, Node>::new(self.2),
		)
	}
}

pub struct RawProp4<T1: IntoProp, T2: IntoProp, T3: IntoProp, T4: IntoProp>(
	pub T1,
	pub T2,
	pub T3,
	pub T4,
);

impl<T1: IntoProp, T2: IntoProp, T3: IntoProp, T4: IntoProp> IntoPropBundle
	for RawProp4<T1, T2, T3, T4>
{
	fn into_bundle<Node: AiNode>(self) -> impl Bundle {
		(
			Prop::<_, Node>::new(self.0),
			Prop::<_, Node>::new(self.1),
			Prop::<_, Node>::new(self.2),
			Prop::<_, Node>::new(self.3),
		)
	}
}

pub struct RawProp5<
	T1: IntoProp,
	T2: IntoProp,
	T3: IntoProp,
	T4: IntoProp,
	T5: IntoProp,
>(pub T1, pub T2, pub T3, pub T4, pub T5);

impl<T1: IntoProp, T2: IntoProp, T3: IntoProp, T4: IntoProp, T5: IntoProp>
	IntoPropBundle for RawProp5<T1, T2, T3, T4, T5>
{
	fn into_bundle<Node: AiNode>(self) -> impl Bundle {
		(
			Prop::<_, Node>::new(self.0),
			Prop::<_, Node>::new(self.1),
			Prop::<_, Node>::new(self.2),
			Prop::<_, Node>::new(self.3),
			Prop::<_, Node>::new(self.4),
		)
	}
}

pub struct RawProp6<
	T1: IntoProp,
	T2: IntoProp,
	T3: IntoProp,
	T4: IntoProp,
	T5: IntoProp,
	T6: IntoProp,
>(pub T1, pub T2, pub T3, pub T4, pub T5, pub T6);

impl<
		T1: IntoProp,
		T2: IntoProp,
		T3: IntoProp,
		T4: IntoProp,
		T5: IntoProp,
		T6: IntoProp,
	> IntoPropBundle for RawProp6<T1, T2, T3, T4, T5, T6>
{
	fn into_bundle<Node: AiNode>(self) -> impl Bundle {
		(
			Prop::<_, Node>::new(self.0),
			Prop::<_, Node>::new(self.1),
			Prop::<_, Node>::new(self.2),
			Prop::<_, Node>::new(self.3),
			Prop::<_, Node>::new(self.4),
			Prop::<_, Node>::new(self.5),
		)
	}
}

pub struct RawProp7<
	T1: IntoProp,
	T2: IntoProp,
	T3: IntoProp,
	T4: IntoProp,
	T5: IntoProp,
	T6: IntoProp,
	T7: IntoProp,
>(pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7);

impl<
		T1: IntoProp,
		T2: IntoProp,
		T3: IntoProp,
		T4: IntoProp,
		T5: IntoProp,
		T6: IntoProp,
		T7: IntoProp,
	> IntoPropBundle for RawProp7<T1, T2, T3, T4, T5, T6, T7>
{
	fn into_bundle<Node: AiNode>(self) -> impl Bundle {
		(
			Prop::<_, Node>::new(self.0),
			Prop::<_, Node>::new(self.1),
			Prop::<_, Node>::new(self.2),
			Prop::<_, Node>::new(self.3),
			Prop::<_, Node>::new(self.4),
			Prop::<_, Node>::new(self.5),
			Prop::<_, Node>::new(self.6),
		)
	}
}

pub struct RawProp8<
	T1: IntoProp,
	T2: IntoProp,
	T3: IntoProp,
	T4: IntoProp,
	T5: IntoProp,
	T6: IntoProp,
	T7: IntoProp,
	T8: IntoProp,
>(
	pub T1,
	pub T2,
	pub T3,
	pub T4,
	pub T5,
	pub T6,
	pub T7,
	pub T8,
);

impl<
		T1: IntoProp,
		T2: IntoProp,
		T3: IntoProp,
		T4: IntoProp,
		T5: IntoProp,
		T6: IntoProp,
		T7: IntoProp,
		T8: IntoProp,
	> IntoPropBundle for RawProp8<T1, T2, T3, T4, T5, T6, T7, T8>
{
	fn into_bundle<Node: AiNode>(self) -> impl Bundle {
		(
			Prop::<_, Node>::new(self.0),
			Prop::<_, Node>::new(self.1),
			Prop::<_, Node>::new(self.2),
			Prop::<_, Node>::new(self.3),
			Prop::<_, Node>::new(self.4),
			Prop::<_, Node>::new(self.5),
			Prop::<_, Node>::new(self.6),
			Prop::<_, Node>::new(self.7),
		)
	}
}

pub struct RawProp9<
	T1: IntoProp,
	T2: IntoProp,
	T3: IntoProp,
	T4: IntoProp,
	T5: IntoProp,
	T6: IntoProp,
	T7: IntoProp,
	T8: IntoProp,
	T9: IntoProp,
>(
	pub T1,
	pub T2,
	pub T3,
	pub T4,
	pub T5,
	pub T6,
	pub T7,
	pub T8,
	pub T9,
);

impl<
		T1: IntoProp,
		T2: IntoProp,
		T3: IntoProp,
		T4: IntoProp,
		T5: IntoProp,
		T6: IntoProp,
		T7: IntoProp,
		T8: IntoProp,
		T9: IntoProp,
	> IntoPropBundle for RawProp9<T1, T2, T3, T4, T5, T6, T7, T8, T9>
{
	fn into_bundle<Node: AiNode>(self) -> impl Bundle {
		(
			Prop::<_, Node>::new(self.0),
			Prop::<_, Node>::new(self.1),
			Prop::<_, Node>::new(self.2),
			Prop::<_, Node>::new(self.3),
			Prop::<_, Node>::new(self.4),
			Prop::<_, Node>::new(self.5),
			Prop::<_, Node>::new(self.6),
			Prop::<_, Node>::new(self.7),
			Prop::<_, Node>::new(self.8),
		)
	}
}

pub struct RawProp10<
	T1: IntoProp,
	T2: IntoProp,
	T3: IntoProp,
	T4: IntoProp,
	T5: IntoProp,
	T6: IntoProp,
	T7: IntoProp,
	T8: IntoProp,
	T9: IntoProp,
	T10: IntoProp,
>(
	pub T1,
	pub T2,
	pub T3,
	pub T4,
	pub T5,
	pub T6,
	pub T7,
	pub T8,
	pub T9,
	pub T10,
);

impl<
		T1: IntoProp,
		T2: IntoProp,
		T3: IntoProp,
		T4: IntoProp,
		T5: IntoProp,
		T6: IntoProp,
		T7: IntoProp,
		T8: IntoProp,
		T9: IntoProp,
		T10: IntoProp,
	> IntoPropBundle for RawProp10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
{
	fn into_bundle<Node: AiNode>(self) -> impl Bundle {
		(
			Prop::<_, Node>::new(self.0),
			Prop::<_, Node>::new(self.1),
			Prop::<_, Node>::new(self.2),
			Prop::<_, Node>::new(self.3),
			Prop::<_, Node>::new(self.4),
			Prop::<_, Node>::new(self.5),
			Prop::<_, Node>::new(self.6),
			Prop::<_, Node>::new(self.7),
			Prop::<_, Node>::new(self.8),
			Prop::<_, Node>::new(self.9),
		)
	}
}

pub struct RawProp11<
	T1: IntoProp,
	T2: IntoProp,
	T3: IntoProp,
	T4: IntoProp,
	T5: IntoProp,
	T6: IntoProp,
	T7: IntoProp,
	T8: IntoProp,
	T9: IntoProp,
	T10: IntoProp,
	T11: IntoProp,
>(
	pub T1,
	pub T2,
	pub T3,
	pub T4,
	pub T5,
	pub T6,
	pub T7,
	pub T8,
	pub T9,
	pub T10,
	pub T11,
);

impl<
		T1: IntoProp,
		T2: IntoProp,
		T3: IntoProp,
		T4: IntoProp,
		T5: IntoProp,
		T6: IntoProp,
		T7: IntoProp,
		T8: IntoProp,
		T9: IntoProp,
		T10: IntoProp,
		T11: IntoProp,
	> IntoPropBundle for RawProp11<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
{
	fn into_bundle<Node: AiNode>(self) -> impl Bundle {
		(
			Prop::<_, Node>::new(self.0),
			Prop::<_, Node>::new(self.1),
			Prop::<_, Node>::new(self.2),
			Prop::<_, Node>::new(self.3),
			Prop::<_, Node>::new(self.4),
			Prop::<_, Node>::new(self.5),
			Prop::<_, Node>::new(self.6),
			Prop::<_, Node>::new(self.7),
			Prop::<_, Node>::new(self.8),
			Prop::<_, Node>::new(self.9),
			Prop::<_, Node>::new(self.10),
		)
	}
}

pub struct RawProp12<
	T1: IntoProp,
	T2: IntoProp,
	T3: IntoProp,
	T4: IntoProp,
	T5: IntoProp,
	T6: IntoProp,
	T7: IntoProp,
	T8: IntoProp,
	T9: IntoProp,
	T10: IntoProp,
	T11: IntoProp,
	T12: IntoProp,
>(
	pub T1,
	pub T2,
	pub T3,
	pub T4,
	pub T5,
	pub T6,
	pub T7,
	pub T8,
	pub T9,
	pub T10,
	pub T11,
	pub T12,
);

impl<
		T1: IntoProp,
		T2: IntoProp,
		T3: IntoProp,
		T4: IntoProp,
		T5: IntoProp,
		T6: IntoProp,
		T7: IntoProp,
		T8: IntoProp,
		T9: IntoProp,
		T10: IntoProp,
		T11: IntoProp,
		T12: IntoProp,
	> IntoPropBundle
	for RawProp12<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>
{
	fn into_bundle<Node: AiNode>(self) -> impl Bundle {
		(
			Prop::<_, Node>::new(self.0),
			Prop::<_, Node>::new(self.1),
			Prop::<_, Node>::new(self.2),
			Prop::<_, Node>::new(self.3),
			Prop::<_, Node>::new(self.4),
			Prop::<_, Node>::new(self.5),
			Prop::<_, Node>::new(self.6),
			Prop::<_, Node>::new(self.7),
			Prop::<_, Node>::new(self.8),
			Prop::<_, Node>::new(self.9),
			Prop::<_, Node>::new(self.10),
			Prop::<_, Node>::new(self.11),
		)
	}
}

pub struct RawProp13<
	T1: IntoProp,
	T2: IntoProp,
	T3: IntoProp,
	T4: IntoProp,
	T5: IntoProp,
	T6: IntoProp,
	T7: IntoProp,
	T8: IntoProp,
	T9: IntoProp,
	T10: IntoProp,
	T11: IntoProp,
	T12: IntoProp,
	T13: IntoProp,
>(
	pub T1,
	pub T2,
	pub T3,
	pub T4,
	pub T5,
	pub T6,
	pub T7,
	pub T8,
	pub T9,
	pub T10,
	pub T11,
	pub T12,
	pub T13,
);

impl<
		T1: IntoProp,
		T2: IntoProp,
		T3: IntoProp,
		T4: IntoProp,
		T5: IntoProp,
		T6: IntoProp,
		T7: IntoProp,
		T8: IntoProp,
		T9: IntoProp,
		T10: IntoProp,
		T11: IntoProp,
		T12: IntoProp,
		T13: IntoProp,
	> IntoPropBundle
	for RawProp13<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>
{
	fn into_bundle<Node: AiNode>(self) -> impl Bundle {
		(
			Prop::<_, Node>::new(self.0),
			Prop::<_, Node>::new(self.1),
			Prop::<_, Node>::new(self.2),
			Prop::<_, Node>::new(self.3),
			Prop::<_, Node>::new(self.4),
			Prop::<_, Node>::new(self.5),
			Prop::<_, Node>::new(self.6),
			Prop::<_, Node>::new(self.7),
			Prop::<_, Node>::new(self.8),
			Prop::<_, Node>::new(self.9),
			Prop::<_, Node>::new(self.10),
			Prop::<_, Node>::new(self.11),
			Prop::<_, Node>::new(self.12),
		)
	}
}
pub struct RawProp14<
	T1: IntoProp,
	T2: IntoProp,
	T3: IntoProp,
	T4: IntoProp,
	T5: IntoProp,
	T6: IntoProp,
	T7: IntoProp,
	T8: IntoProp,
	T9: IntoProp,
	T10: IntoProp,
	T11: IntoProp,
	T12: IntoProp,
	T13: IntoProp,
	T14: IntoProp,
>(
	pub T1,
	pub T2,
	pub T3,
	pub T4,
	pub T5,
	pub T6,
	pub T7,
	pub T8,
	pub T9,
	pub T10,
	pub T11,
	pub T12,
	pub T13,
	pub T14,
);

impl<
		T1: IntoProp,
		T2: IntoProp,
		T3: IntoProp,
		T4: IntoProp,
		T5: IntoProp,
		T6: IntoProp,
		T7: IntoProp,
		T8: IntoProp,
		T9: IntoProp,
		T10: IntoProp,
		T11: IntoProp,
		T12: IntoProp,
		T13: IntoProp,
		T14: IntoProp,
	> IntoPropBundle
	for RawProp14<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>
{
	fn into_bundle<Node: AiNode>(self) -> impl Bundle {
		(
			Prop::<_, Node>::new(self.0),
			Prop::<_, Node>::new(self.1),
			Prop::<_, Node>::new(self.2),
			Prop::<_, Node>::new(self.3),
			Prop::<_, Node>::new(self.4),
			Prop::<_, Node>::new(self.5),
			Prop::<_, Node>::new(self.6),
			Prop::<_, Node>::new(self.7),
			Prop::<_, Node>::new(self.8),
			Prop::<_, Node>::new(self.9),
			Prop::<_, Node>::new(self.10),
			Prop::<_, Node>::new(self.11),
			Prop::<_, Node>::new(self.12),
			Prop::<_, Node>::new(self.13),
		)
	}
}
pub struct RawProp15<
	T1: IntoProp,
	T2: IntoProp,
	T3: IntoProp,
	T4: IntoProp,
	T5: IntoProp,
	T6: IntoProp,
	T7: IntoProp,
	T8: IntoProp,
	T9: IntoProp,
	T10: IntoProp,
	T11: IntoProp,
	T12: IntoProp,
	T13: IntoProp,
	T14: IntoProp,
	T15: IntoProp,
>(
	pub T1,
	pub T2,
	pub T3,
	pub T4,
	pub T5,
	pub T6,
	pub T7,
	pub T8,
	pub T9,
	pub T10,
	pub T11,
	pub T12,
	pub T13,
	pub T14,
	pub T15,
);

impl<
		T1: IntoProp,
		T2: IntoProp,
		T3: IntoProp,
		T4: IntoProp,
		T5: IntoProp,
		T6: IntoProp,
		T7: IntoProp,
		T8: IntoProp,
		T9: IntoProp,
		T10: IntoProp,
		T11: IntoProp,
		T12: IntoProp,
		T13: IntoProp,
		T14: IntoProp,
		T15: IntoProp,
	> IntoPropBundle
	for RawProp15<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>
{
	fn into_bundle<Node: AiNode>(self) -> impl Bundle {
		(
			Prop::<_, Node>::new(self.0),
			Prop::<_, Node>::new(self.1),
			Prop::<_, Node>::new(self.2),
			Prop::<_, Node>::new(self.3),
			Prop::<_, Node>::new(self.4),
			Prop::<_, Node>::new(self.5),
			Prop::<_, Node>::new(self.6),
			Prop::<_, Node>::new(self.7),
			Prop::<_, Node>::new(self.8),
			Prop::<_, Node>::new(self.9),
			Prop::<_, Node>::new(self.10),
			Prop::<_, Node>::new(self.11),
			Prop::<_, Node>::new(self.12),
			Prop::<_, Node>::new(self.13),
			Prop::<_, Node>::new(self.14),
		)
	}
}
