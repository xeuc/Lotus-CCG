use bevy::{
    color::palettes::basic::*, prelude::*
};

use crate::GameState;

pub(crate) const STATUS_BAR_HEIGHT: i32 = 36;

#[derive(Component)]
struct GameViewport;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_rot_button)
            .add_systems(Update, position_game_viewport)
            .add_systems(OnEnter(GameState::InUI), (
                spawn_camera,
                spawn_buttons,
            ))
            ;
    }
}


// SETUP

/// ui camera
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        DespawnOnExit(GameState::InUI)
    ));
}
// Global across every states
fn spawn_rot_button(mut commands: Commands) {
    commands
        .spawn((
            GameViewport,
            Node {
                position_type: PositionType::Absolute,
                top: px(0),
                right: px(0),
                bottom: px(0),
                left: px(0),
                ..default()
            },
            Pickable { should_block_lower: false, is_hoverable: false },
        ))
        .with_children(|viewport| {
            viewport
                .spawn((
                    Button,
                    Node {
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        right: px(10),
                        top: px(8),
                        ..default()
                    },
                    BackgroundColor(WHITE.into()),
                    Pickable { should_block_lower: false, is_hoverable: false },
                ))
                .observe(switch_orientation::<Pointer<Press>>())
                .observe(set_bg_on::<Pointer<Press>>(GREEN.into()))
                .observe(set_bg_on::<Pointer<Release>>(GRAY.into()))
                .observe(set_bg_on::<Pointer<Over>>(GRAY.into()))
                .observe(set_bg_on::<Pointer<Out>>(WHITE.into()))
                .with_child((
                    Text::new("Rot"),
                    TextFont { font_size: 30.0, ..default() },
                    TextColor::BLACK,
                    TextLayout::new_with_justify(Justify::Center),
                ));
        });
}

fn position_game_viewport(
    state: Res<State<GameState>>,
    mut viewport: Query<&mut Node, With<GameViewport>>,
) {
    let top = if *state.get() == GameState::PackShowcase {
        px(STATUS_BAR_HEIGHT)
    } else {
        px(0)
    };

    for mut node in &mut viewport {
        if node.top != top {
            node.top = top;
        }
    }
}


pub fn spawn_buttons(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(GameState::InUI),
            Pickable { should_block_lower: false, is_hoverable: false }, // omg this does smtg
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::RowReverse,
                flex_wrap: FlexWrap::WrapReverse,
                align_content: AlignContent::FlexStart,
                row_gap: px(10),
                column_gap: px(10),
                height: percent(100),
                ..default()
            },
        ))
        .with_children(|parent | {


            // spawn button test ui
            parent.spawn((
                // Pickable { should_block_lower: false, ..default() }, // omg this does smtg
                Button,
                DespawnOnExit(GameState::InUI),
                Text::new("Pack Showcase"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor::BLACK,
                TextLayout::new_with_justify(Justify::Center),
                BackgroundColor(WHITE.into()),
            ))
            .observe(set_game_state_on::<Pointer<Press>>(GameState::PackShowcase))
            .observe(set_bg_on::<Pointer<Press>>(GREEN.into()))
            .observe(set_bg_on::<Pointer<Release>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Over>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Out>>(WHITE.into()))
            ;

            // Open the other prototype.
            parent.spawn((
                Button,
                DespawnOnExit(GameState::InUI),
                Text::new("Other Sppd"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor::BLACK,
                TextLayout::new_with_justify(Justify::Center),
                BackgroundColor(WHITE.into()),
            ))
            .observe(set_game_state_on::<Pointer<Press>>(GameState::OtherSppd))
            .observe(set_bg_on::<Pointer<Press>>(GREEN.into()))
            .observe(set_bg_on::<Pointer<Release>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Over>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Out>>(WHITE.into()))
            ;

            // exit button
            parent.spawn((
                Button,
                DespawnOnExit(GameState::InUI),
                Text::new("Exit"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor::BLACK,
                TextLayout::new_with_justify(Justify::Center),
                BackgroundColor(WHITE.into()),
            ))
            .observe(close_app_on::<Pointer<Press>>())
            .observe(set_bg_on::<Pointer<Press>>(GREEN.into()))
            .observe(set_bg_on::<Pointer<Release>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Over>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Out>>(WHITE.into()))
            ;

        });
}


pub fn set_game_state_on<E: EntityEvent>(
    new_state: GameState,
) -> impl Fn(On<E>, ResMut<NextState<GameState>>) {
    move |_, mut next| { next.set(new_state); }
}

pub fn close_app_on<E: EntityEvent>() -> impl Fn(On<E>, MessageWriter<AppExit>) {
    move |_, mut app_exit_events| { app_exit_events.write(AppExit::Success); }
}

pub fn set_bg_on<E: EntityEvent>(
    color: BackgroundColor,
) -> impl Fn(On<E>, Query<&mut BackgroundColor>) {
    move |event, mut query| {
        if let Ok(mut bg) = query.get_mut(event.event_target()) {
            *bg = color.clone();
        }
    }
}

pub fn switch_orientation<E: EntityEvent>() -> impl FnMut(On<E>, Single<&mut Window>) {
    move |_, mut windows| {
        let (h, w) = (windows.resolution.height() as f32, windows.resolution.width() as f32);
        windows.resolution.set(
            h,
            w,
        );
    }
}
