use std::f32::consts::PI;

use bevy::{color::palettes::basic::*, prelude::*};

use crate::{
    GameState,
    gameplay::PlayerProgress,
    user_interface::STATUS_BAR_HEIGHT,
};

#[derive(Component)]
pub(super) struct StatusStats;

pub fn spawn_buttons(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(GameState::PackShowcase),
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
            Node {
                display: Display::Flex,
                position_type: PositionType::Absolute,
                left: px(20),
                right: px(20),
                bottom: px(20),
                flex_direction: FlexDirection::RowReverse,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Text::new("=>"),
                    TextFont {
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor::BLACK,
                    TextLayout::new_with_justify(Justify::Center),
                    BackgroundColor(WHITE.into()),
                    Pickable {
                        should_block_lower: true,
                        is_hoverable: true,
                    },
                ))
                .observe(look_right_on::<Pointer<Press>>())
                .observe(set_bg_on::<Pointer<Press>>(GREEN.into()))
                .observe(set_bg_on::<Pointer<Release>>(GRAY.into()))
                .observe(set_bg_on::<Pointer<Over>>(GRAY.into()))
                .observe(set_bg_on::<Pointer<Out>>(WHITE.into()));

            parent
                .spawn((
                    Button,
                    Text::new("<="),
                    TextFont {
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor::BLACK,
                    TextLayout::new_with_justify(Justify::Center),
                    BackgroundColor(WHITE.into()),
                    Pickable {
                        should_block_lower: true,
                        is_hoverable: true,
                    },
                ))
                .observe(look_left_on::<Pointer<Press>>())
                .observe(set_bg_on::<Pointer<Press>>(GREEN.into()))
                .observe(set_bg_on::<Pointer<Release>>(GRAY.into()))
                .observe(set_bg_on::<Pointer<Over>>(GRAY.into()))
                .observe(set_bg_on::<Pointer<Out>>(WHITE.into()));
        });

    commands
        .spawn((
            DespawnOnExit(GameState::PackShowcase),
            Node {
                position_type: PositionType::Absolute,
                left: px(0),
                right: px(0),
                top: px(0),
                height: px(STATUS_BAR_HEIGHT),
                padding: UiRect::horizontal(px(10)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.36, 0.18, 0.08)),
        ))
        .with_children(|bar| {
            bar.spawn((
                Text::new("01:23"),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            bar.spawn((
                Text::new(""),
                StatusStats,
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub(super) fn update_status_bar(
    progress: Res<PlayerProgress>,
    mut texts: Query<&mut Text, With<StatusStats>>,
) {
    let status = format!(
        "P{}   Karma {} {:.0}%   {}",
        progress.prestige().points(),
        karma_face(progress.karma()),
        progress.karma().percent(),
        format_money(progress.money().balance()),
    );

    for mut text in &mut texts {
        if text.0 != status {
            text.0.clone_from(&status);
        }
    }
}

fn karma_face(karma: crate::gameplay::Karma) -> &'static str {
    match karma.percent() {
        value if value >= 67.0 => ":)",
        value if value >= 34.0 => ":/",
        _ => ":(",
    }
}

fn format_money(amount: u64) -> String {
    let digits = amount.to_string();
    let mut formatted = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            formatted.push('.');
        }
        formatted.push(digit);
    }
    formatted.push('$');
    formatted
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

pub fn look_left_on<E: EntityEvent>() -> impl FnMut(On<E>, Single<&mut Transform, With<Camera3d>>) {
    move |_, mut cam_transform| cam_transform.rotate_y(PI / 2.0)
}

pub fn look_right_on<E: EntityEvent>() -> impl FnMut(On<E>, Single<&mut Transform, With<Camera3d>>) {
    move |_, mut cam_transform| cam_transform.rotate_y(-PI / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gameplay::Karma;

    #[test]
    fn hud_formats_money_with_dot_grouping() {
        assert_eq!(format_money(153_789), "153.789$");
    }

    #[test]
    fn hud_uses_face_for_karma_range() {
        assert_eq!(karma_face(Karma::new(80.0).unwrap()), ":)");
        assert_eq!(karma_face(Karma::new(50.0).unwrap()), ":/");
        assert_eq!(karma_face(Karma::new(20.0).unwrap()), ":(");
    }
}
