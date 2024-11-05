use proc_macro::TokenStream;
use quote::quote;

/// Derive macro for adding convenient event constructors to an audio channel.
#[proc_macro_derive(AudioChannel)]
pub fn derive_channel(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    // let play_type_name = format_ident!("{}PlayEvent", name);
    // let track_type_name = format_ident!("{}TrackEvent", name);
    // let volume_type_name = format_ident!("{}VolumeEvent", name);
    // let default_type_name = format_ident!("{}DefaultSettingsEvent", name);

    let expanded = quote! {
        // pub type #play_type_name = PlayEvent<#name>;
        // pub type #track_type_name = TrackEvent<#name>;
        // pub type #volume_type_name = VolumeEvent<#name>;
        // pub type #default_type_name = DefaultSettingsEvent<#name>;

        impl AudioChannel for #name {
            fn play_event(id: bevy_audio_controller::audio_files::AudioFiles) -> bevy_audio_controller::events::PlayEvent<#name> {
                bevy_audio_controller::events::PlayEvent::new(id)
            }
            fn settings_event() -> bevy_audio_controller::events::SettingsEvent<#name> {
                bevy_audio_controller::events::SettingsEvent::new()
            }
        }
    };

    TokenStream::from(expanded)
}
