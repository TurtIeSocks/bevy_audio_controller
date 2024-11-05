use bevy::ecs::system::Res;

use crate::{bounds::ACBounds, global::GlobalChannel, resources::ChannelSettings};

pub fn get_normalized_volume<Channel: ACBounds>(
    channel: &Res<ChannelSettings<Channel>>,
    global: &Res<ChannelSettings<GlobalChannel>>,
) -> f32 {
    channel.get_channel_volume() * global.get_channel_volume()
}
