use bevy::{light::DirectionalLightShadowMap, prelude::*};
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin, FreeCameraState};



mod user_interface;
mod open_pack;
mod open_card;
#[cfg(target_os = "android")]
mod android;
mod dev;
mod other_sppd;
mod core;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    InUI, // Player is in User interface, do not spawn anything yet (except buttons)
    OpeningPack, // Player is oppening the pack that is in ressource
    OpeningPack2,
    #[default]
    DevPlayground,
    OtherSppd, // TODO implement
    Binder, // TODO implement
    DeckBuilder, // TODO implement
    HomeScreen, // TODO implement
}


#[derive(Resource, Default)]
struct _PackIDToOpen(u32);

// the `bevy_main` proc_macro generates the required boilerplate for Android
#[bevy_main]
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Lotus CCG".into(),
                resolution: (360, 780).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins((
            core::audio::AudioPlugin,

            FreeCameraPlugin,
            user_interface::UIPlugin,
            open_card::OpenCardPlugin,
            open_pack::open_pack::OpenPackPlugin,
            
            #[cfg(target_os = "android")]
            android::AndroidPlugin,
            dev::dev_playground::DevPlaygroundPlugin,
            other_sppd::other_sppd::OtherSPPDPlugin,
        ))
        .run();
}
