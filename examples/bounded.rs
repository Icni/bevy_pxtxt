use bevy::prelude::*;
use bevy_asset_loader::asset_collection::{AssetCollection, AssetCollectionApp};
use bevy_pxtxt::{plugin::PxtxtPlugin, pxfont::PxFont, pxtext::{PxText, PxTextBundle, PxTextSection}};

#[derive(AssetCollection, Resource)]
struct PxFontCollection {
    #[asset(path = "moonshock.ron")]
    pub moonshock: Handle<PxFont>,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PxtxtPlugin::default()))
        .init_collection::<PxFontCollection>()
        .insert_resource(Msaa::Off)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    fonts: Res<PxFontCollection>,
    mut commands: Commands,
) {
    const GRAY: Color = Color::srgb(0.5, 0.5, 0.5);

    commands.spawn(Camera2dBundle::default());
    commands.spawn(PxTextBundle {
        text: PxText::from_sections(
            vec![
                PxTextSection::new("Frankenstein")
                    .with_color(GRAY).underlined(),
                PxTextSection::new(" by Mary Shelley\n\n")
                    .with_color(GRAY),
                PxTextSection::new("Chapter 1\n\n")
                    .underlined(),
                PxTextSection::new("I am by birth a Genevese, and my family is one of the most distinguished of that republic. My ancestors had been for many years counsellors and syndics, and my father had filled several public situations with honour and reputation. He was respected by all who knew him for his integrity and indefatigable attention to public business. He passed his younger days perpetually occupied by the affairs of his country; a variety of circumstances had prevented his marrying early, nor was it until the decline of life that he became a husband and the father of a family. ...\n\n"),
                PxTextSection::new("(Bound in a 200x200 box.)")
                    .with_color(GRAY),
            ], fonts.moonshock.clone()
        ).with_bounding_box(UVec2::new(200, 200)),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..Default::default()
    });
}
 