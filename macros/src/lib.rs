use proc_macro::TokenStream;

/// Derive macro for adding convenient event constructors to an audio channel.
#[proc_macro_derive(AudioChannel)]
pub fn derive_channel(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let expanded = quote::quote! {
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
        }
    };

    TokenStream::from(expanded)
}
