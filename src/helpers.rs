use bevy::ecs::{component::Component, system::Res};

use crate::{global_channel::GlobalChannel, resources::ChannelSettings};

pub fn get_normalized_volume<Channel: Component + Default>(
    channel: Res<ChannelSettings<Channel>>,
    global: Res<ChannelSettings<GlobalChannel>>,
) -> f32 {
    channel.get_channel_volume() * global.get_channel_volume()
}
