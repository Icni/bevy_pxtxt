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
    commands.spawn(Camera2dBundle::default());
    commands.spawn(PxTextBundle {
        text: PxText::from_sections(
            vec![
                PxTextSection::new("Here's a ")
                    .with_color(Color::WHITE),
                PxTextSection::new("c")
                    .with_color(Color::hsl(0., 0.9, 0.7)).underlined(),
                PxTextSection::new("o")
                    .with_color(Color::hsl(70., 0.9, 0.7)).underlined(),
                PxTextSection::new("l")
                    .with_color(Color::hsl(170., 0.9, 0.7)).underlined(),
                PxTextSection::new("o")
                    .with_color(Color::hsl(220., 0.9, 0.7)).underlined(),
                PxTextSection::new("r")
                    .with_color(Color::hsl(280., 0.9, 0.7)).underlined(),
                PxTextSection::new("ful example of some pixel text.\n\n")
                    .with_color(Color::WHITE),
                PxTextSection::new("(Using sections for different colors)")
                    .with_color(Color::srgb(0.5, 0.5, 0.5)),
            ], fonts.moonshock.clone()
        ).with_line_spacing(5),
        transform: Transform::from_scale(Vec3::splat(4.0)),
        ..Default::default()
    });
}
 