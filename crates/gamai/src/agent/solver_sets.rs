use bevy::prelude::*;

// pub trait SolverSets: Send + Sync + Clone + 'static {
pub trait SolverSets {
	fn factor_set(&self) -> impl SystemSet;
	fn solver_set(&self) -> impl SystemSet;
	fn action_set(&self) -> impl SystemSet;
}
