use bevy::prelude::*;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use super::filesystem_watcher::FilesystemWatcher;
use super::WebAssetIo;

/// Add this plugin to bevy to support loading http and https urls.
///
/// Needs to be added before Bevy's `DefaultPlugins`.
/// Also, make sure `AssetPlugin` is not loaded through `DefaultPlugins`.
///
/// # Example
///
/// ```no_run
/// # use bevy::prelude::*;
/// # use bevy_web_asset::WebAssetPlugin;
///
/// let mut app = App::new();
/// // The web asset plugin should be added instead of the `AssetPlugin`
/// // Internally, WebAssetPlugin will create an AssetPlugin and hook into
/// // it in the right places
/// app.add_plugin(WebAssetPlugin::default());
/// app.add_plugins(DefaultPlugins.build().disable::<AssetPlugin>());
/// ```
///});
#[derive(Default)]
pub struct WebAssetPlugin {
    /// Settings for the underlying (regular) AssetPlugin
    pub asset_plugin: AssetPlugin,
}

impl Plugin for WebAssetPlugin {
    fn build(&self, app: &mut App) {
        // First, configure the underlying plugin
        // We use out own watcher, so `watch_for_changes` is always false
        let asset_plugin = AssetPlugin {
            asset_folder: self.asset_plugin.asset_folder.clone(),
            watch_for_changes: false,
        };

        // Create the `FileAssetIo` wrapper
        let asset_io = {
            // This makes calling `WebAssetIo::watch_for_changes` redundant
            let filesystem_watcher = match self.asset_plugin.watch_for_changes {
                true => Arc::new(RwLock::new(Some(FilesystemWatcher::default()))),
                false => Arc::new(RwLock::new(None)),
            };

            // Create the `FileAssetIo`
            let default_io = asset_plugin.create_platform_default_asset_io();

            // The method doesn't change, so we just use `FileAssetIo`'s
            // let root_path = get_base_path().join(&self.asset_plugin.asset_folder);
						let root_path = "".into();
            WebAssetIo {
                default_io,
                root_path,
                filesystem_watcher,
            }
        };

        // Add the asset server with our `WebAssetIo` wrapping `FileAssetIo`
        app.insert_resource(AssetServer::new(asset_io));

        // Add the asset plugin
        app.add_plugin(asset_plugin);

        // Optionally add the filesystem watcher system
        if self.asset_plugin.watch_for_changes {
            app.add_system(
                super::filesystem_watcher::filesystem_watcher_system
                    .in_base_set(CoreSet::PostUpdate),
            );
        }
    }
}

pub fn get_base_path() -> PathBuf {
	if let Ok(env_bevy_asset_root) = env::var("BEVY_ASSET_ROOT") {
			PathBuf::from(env_bevy_asset_root)
	} else if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
			PathBuf::from(manifest_dir)
	} else {
			env::current_exe()
					.map(|path| {
							path.parent()
									.map(|exe_parent_path| exe_parent_path.to_owned())
									.unwrap()
					})
					.unwrap()
	}
}