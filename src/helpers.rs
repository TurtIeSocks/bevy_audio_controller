use bevy::ecs::system::Res;

use crate::{bounds::Bounds, global::GlobalChannel, resources::ChannelSettings};

pub fn get_normalized_volume<Channel: Bounds>(
    channel: Res<ChannelSettings<Channel>>,
    global: Res<ChannelSettings<GlobalChannel>>,
) -> f32 {
    channel.get_channel_volume() * global.get_channel_volume()
}
