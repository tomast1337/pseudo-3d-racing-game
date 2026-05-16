use super::ids::{AssetId, RoadId, SkyId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Biome {
    Tropical,
    Snow,
    City,
}

impl Biome {
    pub fn next(self) -> Self {
        match self {
            Biome::Tropical => Biome::Snow,
            Biome::Snow => Biome::City,
            Biome::City => Biome::Tropical,
        }
    }
}

pub struct BiomeTheme {
    pub sky: SkyId,
    pub road: RoadId,
    pub roadside: &'static [AssetId],
    pub setpiece: Option<AssetId>,
}

pub struct BiomeCatalog;

impl BiomeCatalog {
    pub const TROPICAL: BiomeTheme = BiomeTheme {
        sky: SkyId::Tropical,
        road: RoadId::Beach,
        roadside: &[
            AssetId::SpriteTreePalm,
            AssetId::SpriteShip,
            AssetId::SpriteCliff,
            AssetId::SpriteHouse,
            AssetId::SpriteBillboard,
        ],
        setpiece: Some(AssetId::SpriteSetpieceForest),
    };

    pub const SNOW: BiomeTheme = BiomeTheme {
        sky: SkyId::Winter,
        road: RoadId::Winter,
        roadside: &[
            AssetId::SpriteTreeWinter,
            AssetId::SpriteSnowman,
            AssetId::SpriteSignPenguin,
            AssetId::SpriteCliffWinter,
            AssetId::SpriteTunnelWinter,
        ],
        setpiece: None,
    };

    pub const CITY: BiomeTheme = BiomeTheme {
        sky: SkyId::Night,
        road: RoadId::City,
        roadside: &[
            AssetId::SpriteTreeCity,
            AssetId::SpriteHouseCity,
            AssetId::SpriteTrash,
            AssetId::SpriteTunnelCity,
            AssetId::SpriteZeppelin,
            AssetId::SpriteBillboard,
        ],
        setpiece: Some(AssetId::SpriteSetpieceCity),
    };

    pub fn theme(biome: Biome) -> &'static BiomeTheme {
        match biome {
            Biome::Tropical => &Self::TROPICAL,
            Biome::Snow => &Self::SNOW,
            Biome::City => &Self::CITY,
        }
    }
}
