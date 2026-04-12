


use bevy::{
    color::palettes::basic::*,
    prelude::*,
};

use crate::GameState;


pub fn spawn_buttons(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(GameState::DevPlayground),
            Pickable { should_block_lower: false, is_hoverable: false },
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
            .observe(set_game_state_on::<Pointer<Press>>(GameState::DevPlayground))
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
