use super::biome::Biome;
use super::ids::AssetId;

pub struct AssetMeta {
    pub id: AssetId,
    pub biomes: &'static [Biome],
}

/// Biome membership for sprite assets (empty = shared across all biomes).
pub const SPRITE_CATALOG: &[AssetMeta] = &[
    meta(AssetId::ShadowDefault, &[]),
    meta(AssetId::MenuHighlight, &[]),
    meta(AssetId::MenuScrollbar, &[]),
    meta(AssetId::MenuSeeker, &[]),
    meta(AssetId::SpriteBarrier, &[]),
    meta(AssetId::SpriteBillboard, &[]),
    meta(AssetId::SpriteBoostReverse, &[]),
    meta(AssetId::SpriteBoost, &[]),
    meta(AssetId::SpriteFinish, &[]),
    meta(AssetId::SpriteFenceBarrier, &[]),
    meta(AssetId::SpriteFence, &[]),
    meta(AssetId::SpriteGuardrail, &[]),
    meta(AssetId::SpriteRampLow, &[]),
    meta(AssetId::SpriteRamp, &[]),
    meta(AssetId::SpriteSign, &[]),
    meta(AssetId::SpriteTeleport, &[]),
    meta(AssetId::SpriteCliff, &[Biome::Tropical]),
    meta(AssetId::SpriteHouse, &[Biome::Tropical]),
    meta(AssetId::SpriteSetpieceForest, &[Biome::Tropical]),
    meta(AssetId::SpriteShip, &[Biome::Tropical]),
    meta(AssetId::SpriteTreePalm, &[Biome::Tropical]),
    meta(AssetId::SpriteCliffWinter, &[Biome::Snow]),
    meta(AssetId::SpriteSignPenguin, &[Biome::Snow]),
    meta(AssetId::SpriteSnowman, &[Biome::Snow]),
    meta(AssetId::SpriteTreeWinter, &[Biome::Snow]),
    meta(AssetId::SpriteTunnelWinter, &[Biome::Snow]),
    meta(AssetId::SpriteHouseCity, &[Biome::City]),
    meta(AssetId::SpriteSetpieceCity, &[Biome::City]),
    meta(AssetId::SpriteTrash, &[Biome::City]),
    meta(AssetId::SpriteTreeCity, &[Biome::City]),
    meta(AssetId::SpriteTunnelCity, &[Biome::City]),
    meta(AssetId::SpriteZeppelin, &[Biome::City]),
    meta(AssetId::SpriteTree, &[]),
    meta(AssetId::SpriteTreeConifer, &[]),
    meta(AssetId::SpriteTreeDead, &[]),
    meta(AssetId::SpriteTunnel, &[]),
];

const fn meta(id: AssetId, biomes: &'static [Biome]) -> AssetMeta {
    AssetMeta { id, biomes }
}
