use bevy::prelude::*;

use crate::{input::handle_input_system, pxfont::{PxFont, PxFontLoader}, pxtext::PxTextEvent, render_text::{prepare_text_system, render_text_system}};

#[derive(Default)]
pub struct PxtxtPlugin;

impl Plugin for PxtxtPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_event::<PxTextEvent>()
            .init_asset::<PxFont>()
            .init_asset_loader::<PxFontLoader>()
            .add_systems(Update, (
                prepare_text_system,
                handle_input_system,
            ))
            .add_systems(PostUpdate, render_text_system);
    }
}
