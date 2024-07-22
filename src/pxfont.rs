use std::{fs::File, io::BufReader, path::PathBuf};

use ahash::AHashMap;
use bevy::{asset::{Asset, AssetLoader, AsyncReadExt}, math::URect, reflect::TypePath};
use image::{ImageFormat, Rgba, RgbaImage};
use thiserror::Error;

use crate::pxfontdata::{GlyphWidth, PxFontData};

#[derive(Debug, Clone)]
pub(crate) struct PxGlyph {
    pub(crate) src_rect: URect,
}

#[derive(Asset, TypePath)]
pub struct PxFont {
    pub(crate) name: String,
    pub(crate) source: RgbaImage,
    pub(crate) char_map: AHashMap<char, PxGlyph>,
    pub(crate) ascender: u32,
    pub(crate) descender: u32,
    pub(crate) spacing: u32,
}

#[derive(Debug, Error)]
pub enum PxFontLoadError {
    #[error("An error was encountered parsing the image: {0}")]
    Image(#[from] image::ImageError),
    #[error("An error was encountered loading the PxFontData file: {0}")]
    Io(#[from] std::io::Error),
    #[error("An error was encountered parsing the RON file: {0}")]
    Ron(#[from] ron::error::SpannedError),
    #[error("Unsupported image type {0}")]
    UnsupportedExtension(String),
    #[error("The image file has no extension, so the format cannot be determined.")]
    NoExtensionProvided,
    #[error("The ascender must not be zero.")]
    MissingAscender,
    #[error("The descender must not be zero.")]
    MissingDescender,
}

#[derive(Default)]
pub struct PxFontLoader;

impl AssetLoader for PxFontLoader {
    type Asset = PxFont;
    type Settings = ();
    type Error = PxFontLoadError;

    fn load<'a>(
            &'a self,
            reader: &'a mut bevy::asset::io::Reader,
            _settings: &'a Self::Settings,
            _load_context: &'a mut bevy::asset::LoadContext,
        ) -> impl bevy::utils::ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let data = ron::de::from_bytes::<PxFontData>(&bytes)?;

            if data.ascender == 0 {
                return Err(PxFontLoadError::MissingAscender);
            }

            if data.descender == 0 {
                return Err(PxFontLoadError::MissingDescender);
            }

            let format = match data.image.extension() {
                #[cfg(feature = "png")]
                Some(ext) if ext == "png" => ImageFormat::Png,
                #[cfg(feature = "jpeg")]
                Some(ext) if ext == "jpeg" || ext == "jpg" => ImageFormat::Jpeg,
                #[cfg(feature = "gif")]
                Some(ext) if ext == "gif" => ImageFormat::Gif,
                #[cfg(feature = "tiff")]
                Some(ext) if ext == "tiff" || ext == "tif" => ImageFormat::Tiff,
                Some(ext) => return Err(PxFontLoadError::UnsupportedExtension(
                    ext.to_str().unwrap().to_string()
                )),
                None => return Err(PxFontLoadError::NoExtensionProvided),
            };
            let file = File::open(PathBuf::from("assets").join(data.image))?;
            let mut rdr = BufReader::new(file);
            let image = image::load(&mut rdr, format)?;
            let source = image.into_rgba8();

            let mut char_map = AHashMap::new();
            let mut x = 0;
            let mut y = 0;

            'map_chars: for c in data.char_layout {
                let height = data.ascender + data.descender;
                let width = match data.glyph_width {
                    GlyphWidth::Varied { max, min } => {
                        let mut width = min + 1;
                        let mut i = x + width - 1;

                        loop {
                            let mut is_blank = true;
                            for j in y..=y + height - 1 {
                                if j >= source.height() {
                                    break 'map_chars;
                                }
                                if *source.get_pixel(i, j) != Rgba::from([0, 0, 0, 0]) {
                                    is_blank = false;
                                }
                            }

                            if is_blank {
                                break width - 1;
                            }

                            if width < max {
                                width += 1;
                                i = x + width - 1;
                            } else {
                                break width;
                            }
                        }
                    },
                    GlyphWidth::Monospace(width) => width,
                };

                let src_rect = URect::new(
                    x, y, x + width - 1, y + height - 1,
                );

                char_map.insert(c, PxGlyph {
                    src_rect,
                });

                x += match data.glyph_width {
                    GlyphWidth::Varied { max, min: _ } => max,
                    GlyphWidth::Monospace(width) => width,
                } + data.padding.0;

                if x >= source.width() {
                    x = 0;
                    y += height + data.padding.1;
                    if y >= source.height() {
                        break;
                    }
                }
            }

            Ok(PxFont {
                name: data.name,
                source,
                char_map,
                ascender: data.ascender,
                descender: data.descender,
                spacing: data.spacing,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
