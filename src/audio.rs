use bevy::prelude::*;

pub struct _AudioPlugin;

impl Plugin for _AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                _setup_music,
            ))
            ;
    }
}


fn _setup_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/Windless Slopes.ogg")),
        PlaybackSettings::LOOP,
    ));
}
