#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SkyId {
    Tropical = 0,
    Winter = 1,
    Night = 2,
}

impl SkyId {
    pub const ALL: [SkyId; 3] = [SkyId::Tropical, SkyId::Winter, SkyId::Night];

    pub fn layer(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RoadId {
    Beach = 0,
    Bridge = 1,
    CityCrossing = 2,
    City = 3,
    Dirt = 4,
    Ice = 5,
    Street = 6,
    Winter = 7,
}

impl RoadId {
    pub const ALL: [RoadId; 8] = [
        RoadId::Beach,
        RoadId::Bridge,
        RoadId::CityCrossing,
        RoadId::City,
        RoadId::Dirt,
        RoadId::Ice,
        RoadId::Street,
        RoadId::Winter,
    ];

    pub fn layer(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AssetId {
    CarDefault,
    ShadowDefault,
    MenuHighlight,
    MenuScrollbar,
    MenuSeeker,
    SpriteBarrier,
    SpriteBillboard,
    SpriteBoostReverse,
    SpriteBoost,
    SpriteCliffWinter,
    SpriteCliff,
    SpriteFenceBarrier,
    SpriteFence,
    SpriteFinish,
    SpriteGuardrail,
    SpriteHouseCity,
    SpriteHouse,
    SpriteRampLow,
    SpriteRamp,
    SpriteSetpieceCity,
    SpriteSetpieceForest,
    SpriteShip,
    SpriteSignPenguin,
    SpriteSign,
    SpriteSnowman,
    SpriteTeleport,
    SpriteTrash,
    SpriteTreeCity,
    SpriteTreeConifer,
    SpriteTreeDead,
    SpriteTreePalm,
    SpriteTreeWinter,
    SpriteTree,
    SpriteTunnelCity,
    SpriteTunnelWinter,
    SpriteTunnel,
    SpriteZeppelin,
}

pub const CAR_FRAME_COUNT: u32 = 7;
pub const CAR_FRAME_WIDTH: u32 = 150;
pub const CAR_FRAME_HEIGHT: u32 = 107;
