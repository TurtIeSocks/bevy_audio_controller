use bevy::prelude::*;

pub fn get_text(text: &str, size: f32) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            text,
            TextStyle {
                font_size: size,
                ..Default::default()
            },
        )
        .with_justify(JustifyText::Center),
        ..Default::default()
    }
}

pub fn get_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[allow(unused)] // ???
pub fn despawn_on_change(mut commands: Commands, query: Query<Entity, With<AudioSink>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
