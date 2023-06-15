use bevy::prelude::*;
use bevy::render::render_resource::ShaderRef;

#[macro_export]

macro_rules! forky_shader {
	($name:expr,$id:expr,$root:expr,$local:expr) => {
		ForkyShader {
			inline: false,
			name: $name,
			id: $id,
			handle: HandleUntyped::weak_from_u64(
				bevy::prelude::Shader::TYPE_UUID,
				$id,
			),
			asset_path: concat!($local, $name, ".wgsl"),
			load_asset: |app: &mut bevy::prelude::App,
			             handle: bevy::prelude::HandleUntyped| {
				bevy::asset::load_internal_asset!(
					app,
					handle,
					concat!($root, $local, $name, ".wgsl"),
					Shader::from_wgsl
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
	pub inline: bool,
	pub name: &'static str,
	pub id: u64,
	pub handle: HandleUntyped,
	pub asset_path: &'static str,
	pub load_asset: fn(&mut App, HandleUntyped),
}

impl ForkyShader {
	pub fn is_inline(&self) -> bool {
		self.inline || !cfg!(feature = "shader_debug")
	}

	pub const fn as_inline(mut self) -> Self {
		self.inline = true;
		self
	}
	pub const fn as_internal(mut self) -> Self {
		if !cfg!(feature = "shader_debug_internal") {
			self.inline = true;
		}
		self
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
			value.handle.typed().into()
		} else {
			value.asset_path.into()
		}
	}
}
