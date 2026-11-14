use std::f32::consts::PI;

use bevy::{color::palettes::basic::*, prelude::*};

use crate::{
    GameState,
    gameplay::PlayerProgress,
    user_interface::STATUS_BAR_HEIGHT,
};

#[derive(Component)]
pub(super) struct StatusStats;

#[derive(Component)]
pub(super) struct AreaActionLabel;

#[derive(Resource, Default, PartialEq, Eq)]
pub struct CurrentArea(Area);

#[derive(Default, PartialEq, Eq)]
enum Area {
    #[default]
    Packs,
    Opponents,
    Deck,
    Exit,
}

impl CurrentArea {
    fn turn_left(&mut self) {
        self.0 = match self.0 {
            Area::Packs => Area::Exit,
            Area::Opponents => Area::Packs,
            Area::Deck => Area::Opponents,
            Area::Exit => Area::Deck,
        };
    }

    fn turn_right(&mut self) {
        self.0 = match self.0 {
            Area::Packs => Area::Opponents,
            Area::Opponents => Area::Deck,
            Area::Deck => Area::Exit,
            Area::Exit => Area::Packs,
        };
    }

    fn action(&self) -> &'static str {
        match self.0 {
            Area::Packs => "Open",
            Area::Opponents => "Fight",
            Area::Deck => "ManageDec",
            Area::Exit => "Exit",
        }
    }

    fn activate(&self, next_state: &mut NextState<GameState>, app_exit: &mut MessageWriter<AppExit>) {
        match self.0 {
            Area::Packs => next_state.set(GameState::OpeningPack2),
            Area::Opponents => next_state.set(GameState::OtherSppd),
            Area::Deck => next_state.set(GameState::DeckBuilder),
            Area::Exit => {
                app_exit.write(AppExit::Success);
            }
        }
    }
}

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
                left: px(12),
                right: px(12),
                bottom: px(14),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            let mut left = parent.spawn(button_bundle());
            left.with_child(button_text_bundle("<=", 26.0));
            left.observe(look_left_on::<Pointer<Press>>());
            add_button_feedback(&mut left);

            let mut action = parent.spawn(button_bundle());
            action.with_child((button_text_bundle("Open", 15.0), AreaActionLabel));
            action.observe(activate_area_action::<Pointer<Press>>());
            add_button_feedback(&mut action);

            let mut right = parent.spawn(button_bundle());
            right.with_child(button_text_bundle("=>", 26.0));
            right.observe(look_right_on::<Pointer<Press>>());
            add_button_feedback(&mut right);
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

fn button_bundle() -> impl Bundle {
    (
        Button,
        Node {
            width: px(104),
            height: px(44),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(WHITE.into()),
        Pickable {
            should_block_lower: true,
            is_hoverable: true,
        },
    )
}

fn button_text_bundle(label: &str, font_size: f32) -> impl Bundle {
    (
        Text::new(label),
        TextFont {
            font_size,
            ..default()
        },
        TextColor::BLACK,
        TextLayout::new_with_justify(Justify::Center),
        Pickable {
            should_block_lower: false,
            is_hoverable: false,
        },
    )
}

fn add_button_feedback(button: &mut EntityCommands) {
    button
        .observe(set_bg_on::<Pointer<Press>>(GREEN.into()))
        .observe(set_bg_on::<Pointer<Release>>(GRAY.into()))
        .observe(set_bg_on::<Pointer<Over>>(GRAY.into()))
        .observe(set_bg_on::<Pointer<Out>>(WHITE.into()));
}

pub(super) fn reset_current_area(mut area: ResMut<CurrentArea>) {
    *area = CurrentArea::default();
}

pub(super) fn update_area_action_label(
    area: Res<CurrentArea>,
    mut labels: Query<&mut Text, With<AreaActionLabel>>,
) {
    if !area.is_changed() {
        return;
    }

    for mut label in &mut labels {
        label.0 = area.action().to_owned();
    }
}

fn activate_area_action<E: EntityEvent>() -> impl Fn(On<E>, Res<CurrentArea>, ResMut<NextState<GameState>>, MessageWriter<AppExit>) {
    move |_, area, mut next_state, mut app_exit| area.activate(&mut next_state, &mut app_exit)
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

pub fn look_left_on<E: EntityEvent>() -> impl FnMut(On<E>, Single<&mut Transform, With<Camera3d>>, ResMut<CurrentArea>) {
    move |_, mut cam_transform, mut area| {
        cam_transform.rotate_y(PI / 2.0);
        area.turn_left();
    }
}

pub fn look_right_on<E: EntityEvent>() -> impl FnMut(On<E>, Single<&mut Transform, With<Camera3d>>, ResMut<CurrentArea>) {
    move |_, mut cam_transform, mut area| {
        cam_transform.rotate_y(-PI / 2.0);
        area.turn_right();
    }
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

    #[test]
    fn area_actions_follow_camera_orientation_and_wrap_around() {
        let mut area = CurrentArea::default();
        assert_eq!(area.action(), "Open");

        area.turn_right();
        assert_eq!(area.action(), "Fight");

        area.turn_right();
        assert_eq!(area.action(), "ManageDec");

        area.turn_right();
        assert_eq!(area.action(), "Exit");

        area.turn_right();
        assert_eq!(area.action(), "Open");

        area.turn_left();
        assert_eq!(area.action(), "Exit");
    }
}
