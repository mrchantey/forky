// Recursive expansion of define_node! macro
// ==========================================

use gamai:: * ;
use gamai::exports:: * ;
#[derive(Debug,Clone,Default,Hash,PartialEq,Eq)]
pub struct Node1<Path:TreePath,System:IntoNodeSystem,Child0:AiNode, >{
  phantom:std::marker::PhantomData<Path> ,system:System,child0:Child0,
}
impl <Path:TreePath,System:IntoNodeSystem,Child0:AiNode, >Node1<Path,System,Child0, >{
  pub fn new<IntoChildMarker0, >(system:System,child0:impl IntoNode<IntoChildMarker0,Out = Child0> ,) -> Self {
    Self {
      system,phantom:std::marker::PhantomData,child0:child0.into_node(),
    }
  }

  }
impl <Path:TreePath,System:IntoNodeSystem,Child0:AiNode, >TreePath for Node1<Path,System,Child0, >{
  type Parent = Path::Parent;
  const CHILD_INDEX:usize = Path::CHILD_INDEX;
}
impl <Path:TreePath,System:IntoNodeSystem,Child0:AiNode, >AiNode for Node1<Path,System,Child0, >{
  type ChildQuery<T:IntoNodeComponent>  = (Entity,(&'static NodeComponent<T,Child0> ,));
  type ChildQueryOpt<T:IntoNodeComponent>  = (Entity,(Option< &'static NodeComponent<T,Child0>> ,));
  type ChildQueryMut<T:IntoNodeComponent>  = (Entity,(&'static mut NodeComponent<T,Child0> ,));
  type ChildQueryOptMut<T:IntoNodeComponent>  = (Entity,(Option< &'static mut NodeComponent<T,Child0>> ,));
  #[allow(unused_parens)]
  type TreeBundle<T:IntoNodeComponent>  = (NodeComponent<T,Self> ,(Child0::TreeBundle<T> ,));
  fn tree_bundle<T:IntoNodeComponent+Clone>(value:T) -> Self::TreeBundle<T>{
    (NodeComponent::new(value.clone()),(Child0::tree_bundle:: <T>(value.clone()),))
  }
  fn add_systems(self,schedule: &mut Schedule){
    Self::configure_sets(schedule);
    schedule.add_systems(self.system.into_node_system_configs:: <Self>());
    self.child0.add_systems(schedule);
  }
  fn entity<'a,T:IntoNodeComponent>(val: & <Self::ChildQuery<T>as bevy_ecs::query::WorldQuery> ::Item<'a>) -> Entity {
    val.0
  }
  fn children<'a,T:IntoNodeComponent>((entity,(value0,)): <Self::ChildQuery<T>as bevy_ecs::query::WorldQuery> ::Item<'a>) -> Vec<ChildState<'a,NodeComponent<T,Self>> >{
    // let value0 =  &value0.into_inner();
		vec![ChildState:: <T>{
      entity:entity.clone(),index:0usize,value:value0,
    }]
  }
  fn children_opt<'a,T:IntoNodeComponent>(item: <Self::ChildQueryOpt<T>as bevy_ecs::query::WorldQuery> ::Item<'a> ,) -> Vec<ChildStateOpt<'a,NodeComponent<T,Self>> >{
    ($crate::vec::Vec::new())
  }
  fn get_child(&self,index:usize) ->  &dyn NodeInspector {
    match index {
      0usize =>  &self.child0,
      _ => {
        $crate::panicking::panic_fmt(builtin #format_args("invalid child index"));
      }
    
      }
  }
  fn get_child_owned(self,index:usize) -> Box<dyn NodeInspector>{
    match index {
      0usize => Box::new(self.child0),
      _ => {
        $crate::panicking::panic_fmt(builtin #format_args("invalid child index"));
      }
    
      }
  }
  fn get_children(&self) -> Vec< &dyn NodeInspector>{
    (<[_]>::into_vec(#[rustc_box]
    $crate::boxed::Box::new([(&self.child0)])))
  }
  fn get_children_owned(self) -> Vec<Box<dyn NodeInspector>>{
    (<[_]>::into_vec(#[rustc_box]
    $crate::boxed::Box::new([(Box::new(self.child0))])))
  }
  fn get_recursive_inner<T:IntoNodeComponent>(self,world: &World,entity:Entity,depth:usize,) -> NodeComponentRecursive<T>{
    NodeComponentRecursive {
      depth,value:NodeComponent:: <T,Self> ::get_ref_from_node(world,entity).map(|v| &v.value),children:(<[_]>::into_vec(#[rustc_box]
      $crate::boxed::Box::new([(self.child0.get_recursive_inner:: <T>(world,entity,depth+1))])))
    }
  }
  fn into_child<NewPath:TreePath>(self) -> impl AiNode {
    Node1:: <NewPath,_,_, > ::new(self.system,self.child0.into_child:: <TreePathSegment<0usize,NewPath>>(),)
  }

  }