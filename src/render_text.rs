use bevy::{prelude::*, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}, texture::ImageSampler}};
use image::{GenericImage, GenericImageView, Rgba, RgbaImage};

use crate::{pxfont::PxFont, pxtext::{PickRect, PickableText, PxText, WrapMode}};

pub(crate) fn prepare_text_system(
    mut images: ResMut<Assets<Image>>,
    q_text: Query<Entity, Without<Handle<Image>>>,
    mut commands: Commands,
) {
    for entity in q_text.iter() {
        let handle = images.add(Image::default());
        commands.entity(entity).insert(handle);
    }
}

pub(crate) fn render_text_system(
    fonts: Res<Assets<PxFont>>,
    mut images: ResMut<Assets<Image>>,
    q_text: Query<(&PxText, &Handle<Image>, &Transform, Option<&Children>), Changed<PxText>>,
    q_pickable: Query<&PickableText>,
    mut commands: Commands,
) {
    for (
        text,
        handle,
        transform,
        children
    ) in &q_text {
        let font = fonts.get(&text.font).unwrap();
        let width = text_width(text, font);
        let height = text_height(text, font);

        let mut output = RgbaImage::new(width, height);
        let mut x = 0;
        let mut y = 0;
        
        let mut first_after_space = true;
        let mut last_char = None;
        let mut first_in_section;

        'draw_glyphs: for section in &text.sections {
            first_in_section = true;
            let string = if WrapMode::WrapWord == text.wrap_mode {
                if let Some(bounds) = text.bounding_box {
                    &wrap_words(
                        (x, y),
                        &section.value,
                        font,
                        text.line_spacing,
                        bounds,
                    )
                } else {
                    &section.value
                }
            } else {
                &section.value
            };

            for c in string.chars() {
                if c == '\n' {
                    x = 0;
                    y += font.ascender + font.descender + text.line_spacing;
                    first_after_space = true;
                } else if let Some(glyph) = font.char_map.get(&c) {
                    if first_in_section && last_char == Some(' ') {
                        first_after_space = true;
                    }

                    if x + glyph.src_rect.width() + 1 > output.width() {
                        x = 0;
                        y += font.ascender + font.descender + text.line_spacing;
                        first_after_space = true;
                    }

                    if y + glyph.src_rect.height() + 1 > output.height() {
                        break 'draw_glyphs;
                    }

                    if let Err(e) = output.copy_from(
                        &*font.source.view(
                            glyph.src_rect.min.x,
                            glyph.src_rect.min.y,
                            glyph.src_rect.width() + 1,
                            glyph.src_rect.height() + 1,
                        ), x, y) {
                        error!("Image error: {e}");
                    }

                    let x_min = if first_after_space {
                        x
                    } else {
                        x - 1
                    };

                    if section.underline {
                        let j = if font.descender < 2 {
                            y + font.ascender
                        } else {
                            y + font.ascender + 1
                        };

                        for i in x_min..=x + glyph.src_rect.width() {
                            output[(i, j)] = Rgba::from([255, 255, 255, 255]);
                        }
                    }

                    let rgba = section.color.to_srgba();
                    for i in x_min..=x + glyph.src_rect.width() {
                        for j in y..=y + glyph.src_rect.height() {
                            let px = output[(i, j)];
                            output[(i, j)] = Rgba::from([
                                (rgba.red * px[0] as f32) as u8,
                                (rgba.green * px[1] as f32) as u8,
                                (rgba.blue * px[2] as f32) as u8,
                                (rgba.alpha * px[3] as f32) as u8,
                            ]);
                        }
                    }

                    x += glyph.src_rect.width() + 1 + font.spacing;

                    if first_after_space {
                        first_after_space = false;
                    }

                    last_char = Some(c);
                }

                if first_in_section {
                    first_in_section = false;
                }
            }
        }

        // Render text

        let mut image = Image::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            output.into_vec(),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD
        );
        image.sampler = ImageSampler::nearest();

        let corner = IVec2::new(
            transform.translation.as_ivec3().x - image.size().as_ivec2().x / 2,
            transform.translation.as_ivec3().y + image.size().as_ivec2().y / 2,
        );

        // Draw pick rects

        if let Some(children) = children {
            for child in children.iter() {
                if let Ok(pickable) = q_pickable.get(*child) {
                    let font = fonts.get(&text.font).unwrap();
                    let mut x = corner.x;
                    let mut y = corner.y;
                    let mut idx = 0;
                    let mut max: Option<IVec2> = None;
                    let mut min: Option<IVec2> = None;

                    let mut rects = Vec::new();

                    let (_string, range) = pickable.get_string(text);

                    'make_rect: for section in &text.sections {
                        for c in section.value.chars() {
                            if c == '\n' {
                                if min.is_none() {
                                    min = Some(IVec2::new(
                                        x,
                                        y - (font.ascender + font.descender) as i32
                                    ));
                                }

                                if let Some(max_rect) = max {
                                    if let Some(min_rect) = min {
                                        rects.push(IRect::from_corners(
                                            (max_rect.as_vec2() * transform.scale.truncate()).as_ivec2(),
                                            (min_rect.as_vec2() * transform.scale.truncate()).as_ivec2(),
                                        ));
                                    }
                                }

                                x = corner.x;
                                y -= (font.ascender + font.descender + text.line_spacing) as i32;

                                max = Some(IVec2::new(x, y));
                                min = None;
                            } else if let Some(glyph) = font.char_map.get(&c) {
                                x += (glyph.src_rect.width() + 1 + font.spacing) as i32;

                                if let Some(_) = max {
                                    if idx == range.end - 1 {
                                        min = Some(IVec2::new(
                                            x,
                                            y - (font.ascender + font.descender) as i32
                                        ));
                                        break 'make_rect;
                                    }
                                } else {
                                    if idx == range.start {
                                        max = Some(IVec2::new(x, y));
                                    }
                                }
                                idx += 1;
                            }
                        }
                    }

                    if let Some(max_rect) = max {
                        if let Some(min_rect) = min {
                            rects.push(IRect::from_corners(
                                (max_rect.as_vec2() * transform.scale.truncate()).as_ivec2(),
                                (min_rect.as_vec2() * transform.scale.truncate()).as_ivec2(),
                            ));
                        }
                    }

                    commands.entity(*child).insert(PickRect(rects));
                }
            }
        }

        *images.get_mut(handle).unwrap() = image;
    }
}

fn text_width(text: &PxText, font: &PxFont) -> u32 {
    if let Some(bounds) = text.bounding_box {
        return bounds.x;
    }

    let mut width = 0;
    let mut line_width = 0;
    let mut first_char = true;

    for section in &text.sections {
        for c in section.value.chars() {
            if first_char {
                first_char = false;
            } else {
                line_width += font.spacing;
            }

            if c == '\n' {
                if line_width > width {
                    width = line_width;
                }
                line_width = 0;
                first_char = true;
            } else if let Some(glyph) = font.char_map.get(&c) {
                line_width += glyph.src_rect.width() + 1;
            } else {
                error!("The font {} does not contain the character {c}", font.name);
            }
        }
    }
    
    if line_width > width {
        width = line_width;
    }

    width + 1
}

fn text_height(text: &PxText, font: &PxFont) -> u32 {
    if let Some(bounds) = text.bounding_box {
        return bounds.y;
    }

    (font.ascender + font.descender + text.line_spacing) * (
        text.sections.iter().map(|section|
            section.value.chars().filter(|c| *c == '\n').count() as u32
        ).sum::<u32>() + 1
    )
}

fn wrap_words(
    start: (u32, u32),
    text: &str,
    font: &PxFont,
    line_spacing: u32,
    bounds: UVec2,
) -> String {
    let (mut x, mut y) = start;
    let mut wrapped = String::with_capacity(text.len());

    for word in text.split_inclusive(word_separator) {
        let mut word_width = 0;
        let mut first_char = true;
        for c in word.chars() {
            if first_char {
                first_char = false;
            } else {
                word_width += font.spacing;
            }

            if let Some(glyph) = font.char_map.get(&c) {
                word_width += glyph.src_rect.width() + 1;
            }

        }

        if x + word_width + 1 > bounds.x {
            wrapped += "\n";
            x = word_width;
            y += font.ascender + font.descender + line_spacing;
        } else {
            x += word_width + 1 + font.spacing;
        }

        if word.chars().last() == Some('\n') {
            x = 0;
            y += font.ascender + font.descender + line_spacing;
        }

        if y + font.ascender + font.descender + 1 > bounds.y {
            return wrapped;
        }

        wrapped += word;
    }

    wrapped
}

fn word_separator(c: char) -> bool {
    c.is_whitespace() || c == '-'
}
