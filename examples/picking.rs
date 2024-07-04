use bevy::prelude::*;
use bevy_asset_loader::asset_collection::{AssetCollection, AssetCollectionApp};
use bevy_pxtxt::{plugin::PxtxtPlugin, pxfont::PxFont, pxtext::{PickableText, PxText, PxTextBundle, PxTextSection, PxTextEvent}};

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
        .add_systems(Update, on_click)
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
                PxTextSection::new("This is an example of pickable things.\n")
                    .with_color(Color::WHITE),
                PxTextSection::new("Click me!\n")
                    .with_color(Color::hsl(90., 0.9, 0.6)),
            ], fonts.moonshock.clone()
        ).with_line_spacing(5),
        transform: Transform::from_scale(Vec3::splat(4.0)),
        ..Default::default()
    }).with_children(|children| {
        children.spawn(PickableText::Sections(1..2));
    });
}

fn on_click(
    mut clicked_text_evr: EventReader<PxTextEvent>,
    mut sprite_query: Query<(Entity, &mut Sprite)>,
) {
    for ev in clicked_text_evr.read() {
        if ev.left_clicked() {
            println!("Left clicked!");
            for (entity, mut sprite) in &mut sprite_query {
                if entity == ev.entity {
                    let new_a = sprite.color.a() - 0.1;
                    sprite.color.set_a(new_a);
                }
            }
        } else if ev.right_clicked() {
            println!("Right clicked!");
            for (entity, mut sprite) in &mut sprite_query {
                if entity == ev.entity {
                    let new_a = sprite.color.a() + 0.1;
                    sprite.color.set_a(new_a);
                }
            }
        }
    }
}
