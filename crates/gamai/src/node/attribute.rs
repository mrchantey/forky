// use crate::*;
use crate::*;
use bevy_ecs::prelude::*;
// use bevy_ecs::schedule::SystemConfigs;

pub struct Attributes<
	PreParentUpdate: IntoNodeSystem,
	PreUpdate: IntoNodeSystem,
	Update: IntoNodeSystem,
	PostUpdate: IntoNodeSystem,
> {
	pub pre_parent_update: PreParentUpdate,
	pub pre_update: PreUpdate,
	pub update: Update,
	pub post_update: PostUpdate,
}

impl<
		PreParentUpdate: IntoNodeSystem,
		PreUpdate: IntoNodeSystem,
		Update: IntoNodeSystem,
		PostUpdate: IntoNodeSystem,
	> Attributes<PreParentUpdate, PreUpdate, Update, PostUpdate>
{
	pub fn new(
		pre_parent_update: PreParentUpdate,
		pre_update: PreUpdate,
		update: Update,
		post_update: PostUpdate,
	) -> Self {
		Self {
			pre_parent_update,
			pre_update,
			update,
			post_update,
		}
	}
}

pub type DefaultAttributes =
	Attributes<empty_node, empty_node, empty_node, empty_node>;

impl<
		PreParentUpdate: IntoNodeSystem,
		PreUpdate: IntoNodeSystem,
		Update: IntoNodeSystem,
		PostUpdate: IntoNodeSystem,
	> Default for Attributes<PreParentUpdate, PreUpdate, Update, PostUpdate>
{
	fn default() -> Self {
		Self {
			pre_parent_update: PreParentUpdate::default(),
			pre_update: PreUpdate::default(),
			update: Update::default(),
			post_update: PostUpdate::default(),
		}
	}
}

/// used by plugins to add systems to the schedule
/// Examples are `node_system`, `before_parent_system`, `before_node_system`
pub trait IntoAttributes: 'static + Send + Sync + Default {
	fn add_systems<N: AiNode>(
		self,
		schedule: &mut Schedule,
		set: impl SystemSet,
	);
}

impl<
		PreParentUpdate: IntoNodeSystem,
		PreUpdate: IntoNodeSystem,
		Update: IntoNodeSystem,
		PostUpdate: IntoNodeSystem,
	> IntoAttributes
	for Attributes<PreParentUpdate, PreUpdate, Update, PostUpdate>
{
	fn add_systems<N: AiNode>(
		self,
		schedule: &mut Schedule,
		set: impl SystemSet,
	) {
		schedule.add_systems(
			self.update.into_node_system_configs::<N>().in_set(set),
		);
	}
}

/// Implementation accepting node systems in the order they are given.
/// ie before_parent, before_node, node, after_node
impl<
		BeforeParent: IntoNodeSystem,
		BeforeNode: IntoNodeSystem,
		Node: IntoNodeSystem,
		AfterNode: IntoNodeSystem,
	> IntoAttributes for (BeforeParent, BeforeNode, Node, AfterNode)
{
	fn add_systems<N: AiNode>(
		self,
		schedule: &mut Schedule,
		set: impl SystemSet,
	) {
		schedule
			.add_systems(self.0.into_node_system_configs::<N>().in_set(set));
		todo!("add_systems on lower level");
		// schedule
		// 	.add_systems(self.1.into_node_system_configs::<N>().in_set(set));
		// schedule
		// 	.add_systems(self.2.into_node_system_configs::<N>().in_set(set));
		// schedule
		// 	.add_systems(self.3.into_node_system_configs::<N>().in_set(set));
	}
}


// pub enum Attribute {
// 	NodeSystem(fn()->&dyn IntoNodeSystem),
// 	BeforeParentSystem(Box<dyn IntoNodeSystem>),
// 	// IntoNodeSystem(Box<dyn IntoNodeSystem<T>>),
// }

// pub struct NodeSystemAttribute {
// 	pub config: SystemConfigs,
// }

// impl NodeSystemAttribute {
// 	pub fn add_system(self, schedule: &mut Schedule, set: impl SystemSet) {
// 		schedule.add_systems(self.config.in_set(set));
// 	}
// }
// pub struct BeforeParentSystemAttribute {
// 	pub config: SystemConfigs,
// }

// impl BeforeParentSystemAttribute {
// 	pub fn add_system(self, schedule: &mut Schedule, set: impl SystemSet) {
// 		schedule.add_systems(self.config.in_set(set));
// 	}
// }


// pub trait IntoAttribute<T> {
// 	fn into_attribute<N: AiNode>(self) -> Attribute;
// }

// pub trait IntoAttributeVec<T> {
// 	fn into_attribute_vec<N: AiNode>(self) -> Vec<Attributes>;
// }

// //TODO use macro instead
// impl<Into1, M1> IntoAttributeVec<(Into1, M1)> for Into1
// where
// 	Into1: IntoAttribute<M1>,
// {
// 	fn into_attribute_vec<N: AiNode>(self) -> Vec<Attribute> {
// 		vec![self.into_attribute::<N>()]
// 	}
// }
// impl<Into1, Into2, M1, M2> IntoAttributeVec<(Into1, Into2, M1, M2)>
// 	for (Into1, Into2)
// where
// 	Into1: IntoAttribute<M1>,
// 	Into2: IntoAttribute<M2>,
// {
// 	fn into_attribute_vec<N: AiNode>(self) -> Vec<Attribute> {
// 		vec![self.0.into_attribute::<N>(), self.1.into_attribute::<N>()]
// 	}
// }

// impl<Into1, Into2, Into3, M1, M2, M3>
// 	IntoAttributeVec<(Into1, Into2, Into3, M1, M2, M3)> for (Into1, Into2, Into3)
// where
// 	Into1: IntoAttribute<M1>,
// 	Into2: IntoAttribute<M2>,
// 	Into3: IntoAttribute<M3>,
// {
// 	fn into_attribute_vec<N: AiNode>(self) -> Vec<Attribute> {
// 		vec![
// 			self.0.into_attribute::<N>(),
// 			self.1.into_attribute::<N>(),
// 			self.2.into_attribute::<N>(),
// 		]
// 	}
// }

// impl<Into1, Into2, Into3, Into4, M1, M2, M3, M4>
// 	IntoAttributeVec<(Into1, Into2, Into3, Into4, M1, M2, M3, M4)>
// 	for (Into1, Into2, Into3, Into4)
// where
// 	Into1: IntoAttribute<M1>,
// 	Into2: IntoAttribute<M2>,
// 	Into3: IntoAttribute<M3>,
// 	Into4: IntoAttribute<M4>,
// {
// 	fn into_attribute_vec<N: AiNode>(self) -> Vec<Attribute> {
// 		vec![
// 			self.0.into_attribute::<N>(),
// 			self.1.into_attribute::<N>(),
// 			self.2.into_attribute::<N>(),
// 			self.3.into_attribute::<N>(),
// 		]
// 	}
// }

// impl<Into1, Into2, Into3, Into4, Into5, M1, M2, M3, M4, M5>
// 	IntoAttributeVec<(Into1, Into2, Into3, Into4, Into5, M1, M2, M3, M4, M5)>
// 	for (Into1, Into2, Into3, Into4, Into5)
// where
// 	Into1: IntoAttribute<M1>,
// 	Into2: IntoAttribute<M2>,
// 	Into3: IntoAttribute<M3>,
// 	Into4: IntoAttribute<M4>,
// 	Into5: IntoAttribute<M5>,
// {
// 	fn into_attribute_vec<N: AiNode>(self) -> Vec<Attribute> {
// 		vec![
// 			self.0.into_attribute::<N>(),
// 			self.1.into_attribute::<N>(),
// 			self.2.into_attribute::<N>(),
// 			self.3.into_attribute::<N>(),
// 			self.4.into_attribute::<N>(),
// 		]
// 	}
// }

// impl<Into1, Into2, Into3, Into4, Into5, Into6, M1, M2, M3, M4, M5, M6>
// 	IntoAttributeVec<(
// 		Into1,
// 		Into2,
// 		Into3,
// 		Into4,
// 		Into5,
// 		Into6,
// 		M1,
// 		M2,
// 		M3,
// 		M4,
// 		M5,
// 		M6,
// 	)> for (Into1, Into2, Into3, Into4, Into5, Into6)
// where
// 	Into1: IntoAttribute<M1>,
// 	Into2: IntoAttribute<M2>,
// 	Into3: IntoAttribute<M3>,
// 	Into4: IntoAttribute<M4>,
// 	Into5: IntoAttribute<M5>,
// 	Into6: IntoAttribute<M6>,
// {
// 	fn into_attribute_vec<N: AiNode>(self) -> Vec<Attribute> {
// 		vec![
// 			self.0.into_attribute::<N>(),
// 			self.1.into_attribute::<N>(),
// 			self.2.into_attribute::<N>(),
// 			self.3.into_attribute::<N>(),
// 			self.4.into_attribute::<N>(),
// 			self.5.into_attribute::<N>(),
// 		]
// 	}
// }

// impl<
// 		Into1,
// 		Into2,
// 		Into3,
// 		Into4,
// 		Into5,
// 		Into6,
// 		Into7,
// 		M1,
// 		M2,
// 		M3,
// 		M4,
// 		M5,
// 		M6,
// 		M7,
// 	>
// 	IntoAttributeVec<(
// 		Into1,
// 		Into2,
// 		Into3,
// 		Into4,
// 		Into5,
// 		Into6,
// 		Into7,
// 		M1,
// 		M2,
// 		M3,
// 		M4,
// 		M5,
// 		M6,
// 		M7,
// 	)> for (Into1, Into2, Into3, Into4, Into5, Into6, Into7)
// where
// 	Into1: IntoAttribute<M1>,
// 	Into2: IntoAttribute<M2>,
// 	Into3: IntoAttribute<M3>,
// 	Into4: IntoAttribute<M4>,
// 	Into5: IntoAttribute<M5>,
// 	Into6: IntoAttribute<M6>,
// 	Into7: IntoAttribute<M7>,
// {
// 	fn into_attribute_vec<N: AiNode>(self) -> Vec<Attribute> {
// 		vec![
// 			self.0.into_attribute::<N>(),
// 			self.1.into_attribute::<N>(),
// 			self.2.into_attribute::<N>(),
// 			self.3.into_attribute::<N>(),
// 			self.4.into_attribute::<N>(),
// 			self.5.into_attribute::<N>(),
// 			self.6.into_attribute::<N>(),
// 		]
// 	}
// }

// impl<
// 		Into1,
// 		Into2,
// 		Into3,
// 		Into4,
// 		Into5,
// 		Into6,
// 		Into7,
// 		Into8,
// 		M1,
// 		M2,
// 		M3,
// 		M4,
// 		M5,
// 		M6,
// 		M7,
// 		M8,
// 	>
// 	IntoAttributeVec<(
// 		Into1,
// 		Into2,
// 		Into3,
// 		Into4,
// 		Into5,
// 		Into6,
// 		Into7,
// 		Into8,
// 		M1,
// 		M2,
// 		M3,
// 		M4,
// 		M5,
// 		M6,
// 		M7,
// 		M8,
// 	)> for (Into1, Into2, Into3, Into4, Into5, Into6, Into7, Into8)
// where
// 	Into1: IntoAttribute<M1>,
// 	Into2: IntoAttribute<M2>,
// 	Into3: IntoAttribute<M3>,
// 	Into4: IntoAttribute<M4>,
// 	Into5: IntoAttribute<M5>,
// 	Into6: IntoAttribute<M6>,
// 	Into7: IntoAttribute<M7>,
// 	Into8: IntoAttribute<M8>,
// {
// 	fn into_attribute_vec<N: AiNode>(self) -> Vec<Attribute> {
// 		vec![
// 			self.0.into_attribute::<N>(),
// 			self.1.into_attribute::<N>(),
// 			self.2.into_attribute::<N>(),
// 			self.3.into_attribute::<N>(),
// 			self.4.into_attribute::<N>(),
// 			self.5.into_attribute::<N>(),
// 			self.6.into_attribute::<N>(),
// 			self.7.into_attribute::<N>(),
// 		]
// 	}
// }
