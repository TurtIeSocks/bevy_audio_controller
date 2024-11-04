use proc_macro::TokenStream;
use quote::format_ident;

/// Derive macro for adding convenient event constructors to an audio channel.
#[proc_macro_derive(AudioChannel)]
pub fn derive_channel(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let play_type_name = format_ident!("{}PlayEvent", name);
    let track_type_name = format_ident!("{}TrackEvent", name);
    let volume_type_name = format_ident!("{}VolumeEvent", name);
    let default_type_name = format_ident!("{}DefaultSettingsEvent", name);

    let expanded = quote::quote! {
        pub type #play_type_name = PlayEvent<#name>;
        pub type #track_type_name = TrackEvent<#name>;
        pub type #volume_type_name = VolumeEvent<#name>;
        pub type #default_type_name = DefaultSettingsEvent<#name>;

        impl #name {
            pub fn new_play_event(id: AudioFiles) -> PlayEvent<#name> {
                PlayEvent::new(id)
            }
            pub fn new_track_event(settings: PlaybackSettings) -> TrackEvent<#name> {
                TrackEvent::new(settings)
            }
            pub fn new_volume_event(volume: f32) -> VolumeEvent<#name> {
                VolumeEvent::new(volume)
            }
            pub fn new_default_settings_event(settings: PlaybackSettings) -> DefaultSettingsEvent<#name> {
                DefaultSettingsEvent::new(settings)
            }
        }
    };

    TokenStream::from(expanded)
}
