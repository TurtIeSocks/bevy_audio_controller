use bevy::prelude::*;

pub fn get_text(text: &str, size: f32) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font_size: size,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
    )
}

pub fn get_container() -> impl Bundle {
    Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        display: Display::Flex,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(10.),
        ..Default::default()
    }
}

#[allow(unused)] // ???
pub fn despawn_on_change(mut commands: Commands, query: Query<Entity, With<AudioSink>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
