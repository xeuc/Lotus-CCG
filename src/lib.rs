use std::fs;
use std::path::PathBuf;

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
        .add_systems(Startup, test_save_system)
        .run();
}

use std::io::{Read, Write};


#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

fn save_system(
    #[cfg(target_os = "android")]
    android_app: bevy::ecs::system::NonSend<AndroidApp>,
) {
    #[cfg(target_os = "android")]
    {
        // let internal_path: PathBuf = android_app.internal_data_path().to_path_buf();
        let internal_path: PathBuf = android_app
            .internal_data_path()
            .expect("No internal data path");
        let file_path = internal_path.join("save.txt");

        if file_path.exists() {
            let mut file2 = fs::File::open(&file_path).unwrap();
            let mut content = String::new();
            file2.read_to_string(&mut content).unwrap();
            info!("Loaded save: {}", content);
        } else {
            info!("No save found, creating one...");
        }+

        let new_data = format!("Saved at: {:?}", std::time::SystemTime::now());
        let mut file2 = fs::File::create(&file_path).unwrap();
        file2.write_all(new_data.as_bytes()).unwrap();
        info!("Saved new data!");
    }

}

use directories::ProjectDirs;

fn test_save_system() {
    // org.bevyengine.Lotus_CCG
    if let Some(proj_dirs) = ProjectDirs::from("org", "bevyengine", "Lotus_CCG") {
        let dir = proj_dirs.data_dir();

        fs::create_dir_all(dir).ok();

        let file = dir.join("save.txt");

        if file.exists() {
            let content = fs::read_to_string(&file).unwrap();
            println!("Loaded: {}", content);
        } else {
            println!("No save found");
        }

        let data = format!("Saved at: {:?}", std::time::SystemTime::now());
        fs::write(file, data).unwrap();

        println!("Saved!");
    }
}


