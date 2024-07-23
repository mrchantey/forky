use bevy::prelude::*;
use bevy::render::render_resource::ShaderRef;

#[macro_export]

macro_rules! forky_shader {
	($name:expr,$id:expr,$root:expr,$local:expr) => {
		ForkyShader {
			inline: LoadMode::External,
			name: $name,
			id: $id,
			handle: Handle::<Shader>::weak_from_u128($id),
			asset_path: concat!($local, $name, ".wgsl"),
			load_asset: |app: &mut bevy::prelude::App,
			             handle: bevy::prelude::Handle<Shader>| {
				bevy::asset::load_internal_asset!(
					app,
					handle,
					concat!($root, $local, $name, ".wgsl"),
					Shader::from_wgsl // Shader::from_wgsl
				);
			},
		}
	};
	($name:expr, $id:expr) => {
		forky_shader!($name, $id, "../../assets/", "shaders/")
	};
}
// const A: ForkyShader = forky_shader!("utility", 0);

pub struct ForkyShader {
	pub inline: LoadMode,
	pub name: &'static str,
	pub id: u128,
	pub handle: Handle<Shader>,
	pub asset_path: &'static str,
	pub load_asset: fn(&mut App, Handle<Shader>),
}

pub enum LoadMode {
	External,
	Inline,
	Internal,
}

impl ForkyShader {
	pub const fn mode(mut self, mode: LoadMode) -> Self {
		self.inline = mode;
		self
	}
	pub fn is_inline(&self) -> bool {
		match self.inline {
			LoadMode::External => !cfg!(feature = "shader_debug"),
			LoadMode::Inline => true,
			LoadMode::Internal => !cfg!(feature = "shader_debug_internal"),
		}
	}
	pub fn try_load_inline(&self, app: &mut App) {
		if self.is_inline() {
			(self.load_asset)(app, self.handle.clone());
		}
	}
}

impl From<ForkyShader> for ShaderRef {
	fn from(value: ForkyShader) -> Self {
		if value.is_inline() {
			value.handle.into()
		} else {
			value.asset_path.into()
		}
	}
}
