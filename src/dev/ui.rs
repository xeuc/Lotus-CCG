
use std::f32::consts::PI;

use bevy::{color::palettes::basic::*, prelude::*};
use crate::{GameState, dev::components::*};



// =============================================================================
// UI
// =============================================================================

pub fn spawn_buttons(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(GameState::DevPlayground),
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

            // Return button
            parent.spawn((
                Button,
                BackgroundColor(WHITE.into()),
                Text::new("Return"),
                TextFont { font_size: 30.0, ..default() },
                TextColor::BLACK,
                TextLayout::new_with_justify(Justify::Center),
            ))
            .observe(set_game_state_on::<Pointer<Press>>(GameState::InUI))
            .observe(set_bg_on::<Pointer<Press>>(GREEN.into()))
            .observe(set_bg_on::<Pointer<Release>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Over>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Out>>(WHITE.into()))
            ;


            // Swipe
            parent.spawn((
                Button,
                BackgroundColor(WHITE.into()),
                Text::new("Swipe?"),
                TextFont { font_size: 30.0, ..default() },
                TextColor::BLACK,
                TextLayout::new_with_justify(Justify::Center),
            ))
            .observe(send_swipe_on::<Pointer<Press>>())
            .observe(set_bg_on::<Pointer<Press>>(GREEN.into()))
            .observe(set_bg_on::<Pointer<Release>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Over>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Out>>(WHITE.into()))
            ;

            // Look Right
            parent.spawn((
                Button,
                BackgroundColor(WHITE.into()),
                Text::new("LookRight"),
                TextFont { font_size: 30.0, ..default() },
                TextColor::BLACK,
                TextLayout::new_with_justify(Justify::Center),
            ))
            .observe(look_right_on::<Pointer<Press>>())
            .observe(set_bg_on::<Pointer<Press>>(GREEN.into()))
            .observe(set_bg_on::<Pointer<Release>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Over>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Out>>(WHITE.into()))
            ;


            // Look left 
            parent.spawn((
                Button,
                BackgroundColor(WHITE.into()),
                Text::new("LookLeft"),
                TextFont { font_size: 30.0, ..default() },
                TextColor::BLACK,
                TextLayout::new_with_justify(Justify::Center),
            ))
            .observe(look_left_on::<Pointer<Press>>())
            .observe(set_bg_on::<Pointer<Press>>(GREEN.into()))
            .observe(set_bg_on::<Pointer<Release>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Over>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Out>>(WHITE.into()))
            ;

            // Lock cam
            parent.spawn((
                Button,
                BackgroundColor(WHITE.into()),
                Text::new("LockCam"),
                TextFont { font_size: 30.0, ..default() },
                TextColor::BLACK,
                TextLayout::new_with_justify(Justify::Center),
            ))
            .observe(lock_cam::<Pointer<Press>>())
            .observe(set_bg_on::<Pointer<Press>>(GREEN.into()))
            .observe(set_bg_on::<Pointer<Release>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Over>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Out>>(WHITE.into()))
            ;

            // UnLock cam
            parent.spawn((
                Button,
                BackgroundColor(WHITE.into()),
                Text::new("UnLockCam"),
                TextFont { font_size: 30.0, ..default() },
                TextColor::BLACK,
                TextLayout::new_with_justify(Justify::Center),
            ))
            .observe(unlock_cam::<Pointer<Press>>())
            .observe(set_bg_on::<Pointer<Press>>(GREEN.into()))
            .observe(set_bg_on::<Pointer<Release>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Over>>(GRAY.into()))
            .observe(set_bg_on::<Pointer<Out>>(WHITE.into()))
            ;

            // print Camera info

            // pause game

        });
}




// =============================================================================
// UI HELPERS
// =============================================================================

pub fn set_bg_on<E: EntityEvent>(
    color: BackgroundColor,
) -> impl Fn(On<E>, Single<&mut BackgroundColor>) {
    move |_, mut query| { **query = color.clone(); }
}

pub fn lock_cam<E: EntityEvent>() -> impl FnMut(On<E>, Commands, Single<Entity, (With<Camera3d>, Without<CameraLocked>)>) {
    move |_, mut commands, cam| { commands.entity(*cam).insert(CameraLocked); }
}

pub fn unlock_cam<E: EntityEvent>() -> impl FnMut(On<E>, Commands, Single<Entity, (With<Camera3d>, With<CameraLocked>)>) {
    move |_, mut commands, cam| { commands.entity(*cam).remove::<CameraLocked>(); }
}

pub fn set_game_state_on<E: EntityEvent>(
    new_state: GameState,
) -> impl Fn(On<E>, ResMut<NextState<GameState>>) {
    move |_, mut next| { next.set(new_state); }
}

pub fn send_swipe_on<E: EntityEvent>() -> impl FnMut(On<E>, MessageWriter<SwipeEvent>) {
    move |_, mut sw: MessageWriter<SwipeEvent>| { sw.write(SwipeEvent); }
}


pub fn look_left_on<E: EntityEvent>() -> impl FnMut(On<E>, Single<&mut Transform, With<Camera3d>>) {
    move |_, mut cam_trans_quer| { cam_trans_quer.rotate_y(PI/2.0) }
}

pub fn look_right_on<E: EntityEvent>() -> impl FnMut(On<E>, Single<&mut Transform, With<Camera3d>>) {
    move |_, mut cam_trans_quer| { cam_trans_quer.rotate_y(-PI/2.0) }
}