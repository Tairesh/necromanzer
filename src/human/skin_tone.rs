use colors::Colors;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::convert::TryFrom;
use tetra::graphics::Color;

#[derive(
    serde::Serialize,
    serde::Deserialize,
    num_enum::IntoPrimitive,
    num_enum::TryFromPrimitive,
    Debug,
    Copy,
    Clone,
)]
#[repr(u8)]
pub enum SkinTone {
    PaleIvory,
    WarmIvory,
    Sand,
    RoseBeige,
    Sienna,
    Limestone,
    Beige,
    Amber,
    Honey,
    Band,
    Almond,
    Umber,
    Bronze,
    Golden,
    Espresso,
    Chocolate,
}

impl SkinTone {
    pub const COUNT: u8 = 16;

    pub fn name(&self) -> &str {
        (*self).into()
    }

    pub fn next(&self) -> Self {
        let mut i: u8 = (*self).into();
        if i < Self::COUNT - 1 {
            i += 1;
        } else {
            i = 0;
        }
        Self::try_from(i).unwrap()
    }

    pub fn prev(&self) -> Self {
        let mut i: u8 = (*self).into();
        if i > 0 {
            i -= 1;
        } else {
            i = Self::COUNT - 1;
        }
        Self::try_from(i).unwrap()
    }

    pub fn text_color(&self) -> Color {
        match self {
            SkinTone::Almond
            | SkinTone::Umber
            | SkinTone::Bronze
            | SkinTone::Golden
            | SkinTone::Espresso
            | SkinTone::Chocolate => Colors::LIGHT_YELLOW,
            _ => Colors::DARK_BROWN,
        }
    }
}

impl From<SkinTone> for &str {
    fn from(s: SkinTone) -> Self {
        match s {
            SkinTone::PaleIvory => "Pale Ivory",
            SkinTone::WarmIvory => "Warm Ivory",
            SkinTone::Sand => "Sandy",
            SkinTone::RoseBeige => "Rose Beige",
            SkinTone::Sienna => "Sienna",
            SkinTone::Limestone => "Limestone",
            SkinTone::Beige => "Beige",
            SkinTone::Amber => "Amber",
            SkinTone::Honey => "Honey",
            SkinTone::Band => "Band",
            SkinTone::Almond => "Almond",
            SkinTone::Umber => "Umber",
            SkinTone::Bronze => "Bronze",
            SkinTone::Golden => "Golden",
            SkinTone::Espresso => "Espresso",
            SkinTone::Chocolate => "Chocolate",
        }
    }
}

impl From<SkinTone> for Color {
    fn from(s: SkinTone) -> Self {
        match s {
            SkinTone::PaleIvory => Colors::PALE_IVORY,
            SkinTone::WarmIvory => Colors::WARM_IVORY,
            SkinTone::Sand => Colors::SAND,
            SkinTone::RoseBeige => Colors::ROSE_BEIGE,
            SkinTone::Sienna => Colors::SIENNA,
            SkinTone::Limestone => Colors::LIMESTONE,
            SkinTone::Beige => Colors::BEIGE,
            SkinTone::Amber => Colors::AMBER,
            SkinTone::Honey => Colors::HONEY,
            SkinTone::Band => Colors::BAND,
            SkinTone::Almond => Colors::ALMOND,
            SkinTone::Umber => Colors::UMBER,
            SkinTone::Bronze => Colors::BRONZE,
            SkinTone::Golden => Colors::GOLDEN,
            SkinTone::Espresso => Colors::ESPRESSO,
            SkinTone::Chocolate => Colors::CHOCOLATE,
        }
    }
}

impl Distribution<SkinTone> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SkinTone {
        match rng.gen_range(0..=15) {
            0 => SkinTone::PaleIvory,
            1 => SkinTone::WarmIvory,
            2 => SkinTone::Sand,
            3 => SkinTone::RoseBeige,
            4 => SkinTone::Sienna,
            5 => SkinTone::Limestone,
            6 => SkinTone::Beige,
            7 => SkinTone::Amber,
            8 => SkinTone::Honey,
            9 => SkinTone::Band,
            10 => SkinTone::Almond,
            11 => SkinTone::Umber,
            12 => SkinTone::Bronze,
            13 => SkinTone::Golden,
            14 => SkinTone::Espresso,
            15 => SkinTone::Chocolate,
            _ => {
                panic!("Rust is the memory safe language with zero cost abstractions!");
            }
        }
    }
}
