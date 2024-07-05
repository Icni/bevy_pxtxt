use bevy::prelude::*;

use crate::{input::handle_input_system, pxfont::{PxFont, PxFontLoader}, pxtext::PxTextEvent, render_text::{prepare_text_system, render_text_system}};

pub struct PxtxtPlugin {
    pub pixel_perfect: bool,
}

impl Default for PxtxtPlugin {
    fn default() -> Self {
        Self {
            pixel_perfect: true,
        }
    }
}

impl Plugin for PxtxtPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_event::<PxTextEvent>()
            .init_asset::<PxFont>()
            .init_asset_loader::<PxFontLoader>()
            .add_systems(PreUpdate, prepare_text_system)
            .add_systems(Update, handle_input_system)
            .add_systems(PostUpdate, render_text_system);
        if self.pixel_perfect {
            app.insert_resource(Msaa::Off);
        }
    }
}
