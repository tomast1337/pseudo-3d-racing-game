use super::ids::{AssetId, RoadId, SkyId};

macro_rules! tex {
    ($name:expr) => {
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/textures/",
            $name
        ))
    };
}

pub fn sky_bytes(id: SkyId) -> &'static [u8] {
    match id {
        SkyId::Tropical => tex!("sky_tropical.png"),
        SkyId::Winter => tex!("sky_winter.png"),
        SkyId::Night => tex!("sky_night.png"),
    }
}

pub fn road_bytes(id: RoadId) -> &'static [u8] {
    match id {
        RoadId::Beach => tex!("road_beach.png"),
        RoadId::Bridge => tex!("road_bridge.png"),
        RoadId::CityCrossing => tex!("road_city_crossing.png"),
        RoadId::City => tex!("road_city.png"),
        RoadId::Dirt => tex!("road_dirt.png"),
        RoadId::Ice => tex!("road_ice.png"),
        RoadId::Street => tex!("road_street.png"),
        RoadId::Winter => tex!("road_winter.png"),
    }
}

pub fn car_bytes() -> &'static [u8] {
    tex!("car_default.png")
}

pub fn asset_bytes(id: AssetId) -> &'static [u8] {
    match id {
        AssetId::CarDefault => tex!("car_default.png"),
        AssetId::ShadowDefault => tex!("shadow_default.png"),
        AssetId::MenuHighlight => tex!("menu_highlight.png"),
        AssetId::MenuScrollbar => tex!("menu_scrollbar.png"),
        AssetId::MenuSeeker => tex!("menu_seeker.png"),
        AssetId::SpriteBarrier => tex!("sprite_barrier.png"),
        AssetId::SpriteBillboard => tex!("sprite_billboard.png"),
        AssetId::SpriteBoostReverse => tex!("sprite_boost_reverse.png"),
        AssetId::SpriteBoost => tex!("sprite_boost.png"),
        AssetId::SpriteCliffWinter => tex!("sprite_cliff_winter.png"),
        AssetId::SpriteCliff => tex!("sprite_cliff.png"),
        AssetId::SpriteFenceBarrier => tex!("sprite_fence_barrier.png"),
        AssetId::SpriteFence => tex!("sprite_fence.png"),
        AssetId::SpriteFinish => tex!("sprite_finish.png"),
        AssetId::SpriteGuardrail => tex!("sprite_guardrail.png"),
        AssetId::SpriteHouseCity => tex!("sprite_house_city.png"),
        AssetId::SpriteHouse => tex!("sprite_house.png"),
        AssetId::SpriteRampLow => tex!("sprite_ramp_low.png"),
        AssetId::SpriteRamp => tex!("sprite_ramp.png"),
        AssetId::SpriteSetpieceCity => tex!("sprite_setpiece_city.png"),
        AssetId::SpriteSetpieceForest => tex!("sprite_setpiece_forest.png"),
        AssetId::SpriteShip => tex!("sprite_ship.png"),
        AssetId::SpriteSignPenguin => tex!("sprite_sign_penguin.png"),
        AssetId::SpriteSign => tex!("sprite_sign.png"),
        AssetId::SpriteSnowman => tex!("sprite_snowman.png"),
        AssetId::SpriteTeleport => tex!("sprite_teleport.png"),
        AssetId::SpriteTrash => tex!("sprite_trash.png"),
        AssetId::SpriteTreeCity => tex!("sprite_tree_city.png"),
        AssetId::SpriteTreeConifer => tex!("sprite_tree_conifer.png"),
        AssetId::SpriteTreeDead => tex!("sprite_tree_dead.png"),
        AssetId::SpriteTreePalm => tex!("sprite_tree_palm.png"),
        AssetId::SpriteTreeWinter => tex!("sprite_tree_winter.png"),
        AssetId::SpriteTree => tex!("sprite_tree.png"),
        AssetId::SpriteTunnelCity => tex!("sprite_tunnel_city.png"),
        AssetId::SpriteTunnelWinter => tex!("sprite_tunnel_winter.png"),
        AssetId::SpriteTunnel => tex!("sprite_tunnel.png"),
        AssetId::SpriteZeppelin => tex!("sprite_zeppelin.png"),
    }
}

/// All sprites, menu UI, and shadow packed into the sprite texture array.
pub const SPRITE_ASSETS: &[AssetId] = &[
    AssetId::ShadowDefault,
    AssetId::MenuHighlight,
    AssetId::MenuScrollbar,
    AssetId::MenuSeeker,
    AssetId::SpriteBarrier,
    AssetId::SpriteBillboard,
    AssetId::SpriteBoostReverse,
    AssetId::SpriteBoost,
    AssetId::SpriteCliffWinter,
    AssetId::SpriteCliff,
    AssetId::SpriteFenceBarrier,
    AssetId::SpriteFence,
    AssetId::SpriteFinish,
    AssetId::SpriteGuardrail,
    AssetId::SpriteHouseCity,
    AssetId::SpriteHouse,
    AssetId::SpriteRampLow,
    AssetId::SpriteRamp,
    AssetId::SpriteSetpieceCity,
    AssetId::SpriteSetpieceForest,
    AssetId::SpriteShip,
    AssetId::SpriteSignPenguin,
    AssetId::SpriteSign,
    AssetId::SpriteSnowman,
    AssetId::SpriteTeleport,
    AssetId::SpriteTrash,
    AssetId::SpriteTreeCity,
    AssetId::SpriteTreeConifer,
    AssetId::SpriteTreeDead,
    AssetId::SpriteTreePalm,
    AssetId::SpriteTreeWinter,
    AssetId::SpriteTree,
    AssetId::SpriteTunnelCity,
    AssetId::SpriteTunnelWinter,
    AssetId::SpriteTunnel,
    AssetId::SpriteZeppelin,
];
