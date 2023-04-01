use super::*;
use crate::{spline::Spline, *};
use bevy::{prelude::*, utils::HashMap};
use derive_deref::{Deref, DerefMut};
use forky_core::*;

#[derive(Debug, Clone)]
pub struct SplineGraphHandle {
	pub nodes: HashMap<u64, Entity>,
	pub edges: HashMap<u64, Entity>,
}

impl SplineGraphHandle {}
