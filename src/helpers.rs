use bevy::{audio::Volume, ecs::system::Res};

use crate::{bounds::ACBounds, global::GlobalChannel, resources::ChannelSettings};

pub fn get_normalized_volume<Channel: ACBounds>(
    channel: &Res<ChannelSettings<Channel>>,
    global: &Res<ChannelSettings<GlobalChannel>>,
) -> Volume {
    channel.get_channel_volume() * global.get_channel_volume()
}
