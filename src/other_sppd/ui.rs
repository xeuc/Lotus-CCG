


use bevy::{
    color::palettes::basic::*,
    prelude::*,
};

use crate::GameState;

pub fn setup_ui(
    mut commands: Commands,
) {
    // Instructions
    commands.spawn((
        DespawnOnExit(GameState::OtherSppd),
        Text::new("Left  Click to create a Blue ball that goes right
Right Click to create a Red  ball that goes left
Green circle is detection range
Yellow circle is attack range
Red circle is size range (hitbox or hitball lol)
If balls of different teams encounter, they will attack each others"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}


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

            // return button 
            parent.spawn((
                Button,
                DespawnOnExit(GameState::OtherSppd),
                Text::new("Return"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor::BLACK,
                TextLayout::new_with_justify(Justify::Center),
                BackgroundColor(WHITE.into()),
            ))
            .observe(set_game_state_on::<Pointer<Press>>(GameState::InUI))
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

pub fn set_bg_on<E: EntityEvent>(
    color: BackgroundColor,
) -> impl Fn(On<E>, Query<&mut BackgroundColor>) {
    move |event, mut query| {
        if let Ok(mut bg) = query.get_mut(event.event_target()) {
            *bg = color.clone();
        }
    }
}
