use bevy::app::{PluginGroup, PluginGroupBuilder};

mod camera_controls;

struct CameraSystemPlugins;

impl PluginGroup for CameraSystemPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(camera_controls::CameraControlPlugin)
    }
}

pub(crate) fn build_plugins() -> PluginGroupBuilder {
    CameraSystemPlugins::build(CameraSystemPlugins)
}
