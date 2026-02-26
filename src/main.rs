use bevy::prelude::*;

mod app;
mod move_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(move_camera::CameraControllerPlugin)
        .add_plugins(app::CCGLotusPlugin)
        .run();
}