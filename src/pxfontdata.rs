use std::{ops::RangeInclusive, path::PathBuf};

use serde::{Deserialize, Serialize};

const fn zerozero() -> (u32, u32) { (0, 0) }
const fn one() -> u32 { 1 }

#[derive(Debug, Serialize, Deserialize)]
pub struct PxFontData {
    pub name: String,
    pub image: PathBuf,
    pub glyph_width: GlyphWidth,
    pub ascender: u32,
    pub descender: u32,
    #[serde(default)]
    pub char_layout: CharLayout,
    #[serde(default = "one")]
    pub spacing: u32,
    #[serde(default = "zerozero")]
    pub padding: (u32, u32),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GlyphWidth {
    Varied {
        max: u32,
        min: u32,
    },
    Monospace(u32),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CharLayout {
    StartingAt(char),
    Ranges(Vec<RangeInclusive<char>>),
    Listed(Vec<char>),
}

impl Default for CharLayout {
    fn default() -> Self {
        Self::StartingAt(' ')
    }
}

impl CharLayout {
    pub fn get(&self, index: u32) -> Option<char> {
        match &self {
            Self::StartingAt(c) => {
                let code: u32 = (*c).into();
                let new = code + index;
                char::from_u32(new)
            }
            _ => todo!(),
        }
    }
}

impl IntoIterator for CharLayout {
    type IntoIter = CharLayoutIter;
    type Item = char;

    fn into_iter(self) -> Self::IntoIter {
        let idx = match &self {
            CharLayout::StartingAt(c) => (*c).into(),
            CharLayout::Ranges(vec) => (*vec[0].start()).into(),
            CharLayout::Listed(vec) => vec[0].into(),
        };

        CharLayoutIter {
            layout: self,
            idx,
            started: false,
        }
    }
}

pub struct CharLayoutIter {
    layout: CharLayout,
    idx: char,
    started: bool,
}

impl Iterator for CharLayoutIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            return Some(self.idx)
        }

        match &self.layout {
            CharLayout::StartingAt(_) => {
                let mut cursor = self.idx as u32;
                while cursor < char::MAX as u32 {
                    cursor += 1;
                    if let Some(new) = char::from_u32(cursor) {
                        self.idx = new;
                        return Some(self.idx);
                    }
                }
                None
            },
            CharLayout::Ranges(vec) => {
                if let Some((idx, range)) = vec
                    .iter()
                    .enumerate()
                    .find(|(_, v)| v.contains(&self.idx)) {
                    if *range.end() == self.idx {
                        if let Some(next) = vec.get(idx + 1) {
                            self.idx = *next.start();
                            Some(self.idx)
                        } else {
                            None
                        }
                    } else {
                        let mut cursor = self.idx as u32;
                        while cursor < char::MAX as u32 {
                            cursor += 1;
                            if let Some(new) = char::from_u32(cursor) {
                                self.idx = new;
                                return Some(self.idx);
                            }
                        }
                        None
                    }
                } else {
                    None
                }
            }
            CharLayout::Listed(vec) => {
                if let Some((i, _)) = vec
                    .iter()
                    .enumerate()
                    .find(|(_, v)| **v == self.idx) {
                    vec.get(i + 1).cloned()
                } else {
                    None
                }
            }
        }
    }
}
