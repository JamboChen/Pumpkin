use dimensions::Dimension;
use serde::Serialize;

use super::bytebuf::ByteBuffer;

mod biomes;
mod chat_type;
mod damage_type;
mod dimensions;

#[derive(Debug, Clone, Serialize)]
struct LoginInfo {
    #[serde(rename = "minecraft:dimension_type")]
    dimensions: Codec<dimensions::Dimension>,
    /*   #[serde(rename = "minecraft:worldgen/biome")]
    biomes: Codec<biomes::Biome>,
    #[serde(rename = "minecraft:chat_type")]
    chat: Codec<chat_type::ChatType>,
    #[serde(rename = "minecraft:damage_type")]
    damage: Codec<damage_type::DamageType>, */
}

#[derive(Debug, Clone, Serialize)]
struct Codec<T> {
    #[serde(rename = "type")]
    ty: String,
    value: Vec<RegistryValue<T>>,
}
#[derive(Debug, Clone, Serialize)]
struct RegistryValue<T> {
    name: String,
    id: i32,
    element: T,
}

pub fn write_single_dimension(out: &mut ByteBuffer, world_min_y: i32, world_height: u32) {
    let dimension = Dimension::default();
    let nbt = Codec {
        ty: "minecraft:dimension_type".into(),
        value: vec![RegistryValue {
            name: "minecraft:overworld".into(),
            id: 0,
            element: dimension,
        }],
    };
    let val = &fastsnbt::to_string(&nbt).unwrap();
    dbg!(val);
    out.put_string(val);
}

pub fn write_codec(out: &mut ByteBuffer, world_min_y: i32, world_height: u32) {
    let dimension = Dimension::default();

    let info = LoginInfo {
        dimensions: Codec {
            ty: "minecraft:dimension_type".into(),
            value: vec![RegistryValue {
                name: "minecraft:overworld".into(),
                id: 0,
                element: dimension,
            }],
        },
        /* biomes: Codec {
            ty: "minecraft:worldgen/biome".into(),
            value: biomes::all(),
        },
        chat: Codec {
            ty: "minecraft:chat_type".into(),
            value: chat_type::all(),
        },
        damage: Codec {
            ty: "minecraft:damage_type".into(),
            value: damage_type::all(),
            }, */
    };

    let val = &fastsnbt::to_string(&info).unwrap();
    dbg!(val);
    // Dimension codec
    out.put_string(val);
    // Current dimension type (key in dimension codec)
    out.put_string("minecraft:overworld");
    // Current world
    out.put_string("minecraft:overworld");
}
