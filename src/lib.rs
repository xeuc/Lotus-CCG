//! A 3d Scene with a button and playing sound.
use bevy::{light::DirectionalLightShadowMap, prelude::*};
// cargo ndk -t arm64-v8a -o android_example/app/src/main/jniLibs build

mod user_interface;
mod open_card;
mod audio;
mod input;
#[cfg(target_os = "android")]
mod android;
mod move_camera;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    InUI, // Player is in User interface, do not spawn anything yet (except buttons)
    OpeningPack, // Player is oppening the pack that is in ressource
    Battling, // TODO implement
}


#[derive(Resource, Default)]
struct _PackIDToOpen(u32);


// the `bevy_main` proc_macro generates the required boilerplate for Android
#[bevy_main]
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins((
            // audio::_AudioPlugin,

            user_interface::UIPlugin,
            open_card::OpenCardPlugin,
            
            input::InputPlugin,
            #[cfg(target_os = "android")]
            android::AndroidPlugin,
            move_camera::CameraControllerPlugin,
        ))
        .run();
}

