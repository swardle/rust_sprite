use image::{GenericImageView, ImageBuffer, ImageFormat};
use std::fs;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpriteJson {
    pub file_format_version: i64,
    pub guid: String,
    #[serde(rename = "TextureImporter")]
    pub texture_importer: TextureImporter,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextureImporter {
    #[serde(rename = "internalIDToNameTable")]
    pub internal_idto_name_table: Vec<InternalIdtoNameTable>,
    pub external_objects: ExternalObjects,
    pub serialized_version: i64,
    pub mipmaps: Mipmaps,
    pub bumpmap: Bumpmap,
    pub is_readable: i64,
    pub streaming_mipmaps: i64,
    pub streaming_mipmaps_priority: i64,
    #[serde(rename = "vTOnly")]
    pub v_tonly: i64,
    pub gray_scale_to_alpha: i64,
    pub generate_cubemap: i64,
    pub cubemap_convolution: i64,
    pub seamless_cubemap: i64,
    pub texture_format: i64,
    pub max_texture_size: i64,
    pub texture_settings: TextureSettings,
    #[serde(rename = "nPOTScale")]
    pub n_potscale: i64,
    pub lightmap: i64,
    pub compression_quality: i64,
    pub sprite_mode: i64,
    pub sprite_extrude: i64,
    pub sprite_mesh_type: i64,
    pub alignment: i64,
    pub sprite_pivot: SpritePivot,
    pub sprite_pixels_to_units: i64,
    pub sprite_border: SpriteBorder,
    pub sprite_generate_fallback_physics_shape: i64,
    pub alpha_usage: i64,
    pub alpha_is_transparency: i64,
    pub sprite_tessellation_detail: i64,
    pub texture_type: i64,
    pub texture_shape: i64,
    pub single_channel_component: i64,
    pub flipbook_rows: i64,
    pub flipbook_columns: i64,
    pub max_texture_size_set: i64,
    pub compression_quality_set: i64,
    pub texture_format_set: i64,
    pub ignore_png_gamma: i64,
    pub apply_gamma_decoding: i64,
    pub platform_settings: Vec<PlatformSetting>,
    pub sprite_sheet: SpriteSheet,
    pub sprite_packing_tag: Value,
    #[serde(rename = "pSDRemoveMatte")]
    pub p_sdremove_matte: i64,
    #[serde(rename = "pSDShowRemoveMatteOption")]
    pub p_sdshow_remove_matte_option: i64,
    pub user_data: Value,
    pub asset_bundle_name: Value,
    pub asset_bundle_variant: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalIdtoNameTable {
    pub first: First,
    pub second: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct First {
    #[serde(rename = "213")]
    pub n213: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalObjects {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mipmaps {
    pub mip_map_mode: i64,
    pub enable_mip_map: i64,
    #[serde(rename = "sRGBTexture")]
    pub s_rgbtexture: i64,
    pub linear_texture: i64,
    pub fade_out: i64,
    pub border_mip_map: i64,
    pub mip_maps_preserve_coverage: i64,
    pub alpha_test_reference_value: f64,
    pub mip_map_fade_distance_start: i64,
    pub mip_map_fade_distance_end: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bumpmap {
    pub convert_to_normal_map: i64,
    pub external_normal_map: i64,
    pub height_scale: f64,
    pub normal_map_filter: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextureSettings {
    pub serialized_version: i64,
    pub filter_mode: i64,
    pub aniso: i64,
    pub mip_bias: i64,
    pub wrap_u: i64,
    pub wrap_v: i64,
    pub wrap_w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpritePivot {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpriteBorder {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformSetting {
    pub serialized_version: i64,
    pub build_target: String,
    pub max_texture_size: i64,
    pub resize_algorithm: i64,
    pub texture_format: i64,
    pub texture_compression: i64,
    pub compression_quality: i64,
    pub crunched_compression: i64,
    pub allows_alpha_splitting: i64,
    pub overridden: i64,
    #[serde(rename = "androidETC2FallbackOverride")]
    pub android_etc2fallback_override: i64,
    #[serde(rename = "forceMaximumCompressionQuality_BC6H_BC7")]
    pub force_maximum_compression_quality_bc6h_bc7: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpriteSheet {
    pub serialized_version: i64,
    pub sprites: Vec<Sprite>,
    pub outline: Vec<Value>,
    pub physics_shape: Vec<Value>,
    pub bones: Vec<Value>,
    #[serde(rename = "spriteID")]
    pub sprite_id: String,
    #[serde(rename = "internalID")]
    pub internal_id: i64,
    pub vertices: Vec<Value>,
    pub indices: Value,
    pub edges: Vec<Value>,
    pub weights: Vec<Value>,
    pub secondary_textures: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sprite {
    pub serialized_version: i64,
    pub name: String,
    pub rect: Rect,
    pub alignment: i64,
    pub pivot: Pivot,
    pub border: Border,
    pub outline: Vec<Value>,
    pub physics_shape: Vec<Value>,
    pub tessellation_detail: i64,
    pub bones: Vec<Value>,
    #[serde(rename = "spriteID")]
    pub sprite_id: String,
    #[serde(rename = "internalID")]
    pub internal_id: i64,
    pub vertices: Vec<Value>,
    pub indices: Value,
    pub edges: Vec<Value>,
    pub weights: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rect {
    pub serialized_version: i64,
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pivot {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Border {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub w: i64,
}


fn getCardName(value: usize, suit: &str) -> String {
    let card_name: String;
    if (suit == "Back") {
        card_name = format!("Card {suit}");
    } else {
        card_name = format!("Card_{value:02}_{suit}");
    }
    return card_name;
}

fn indexToValue(index: usize) -> usize {
    let value = (13 - (index % 13)) + 1;
    return value;
}

fn indexToSuit(index: usize) -> &'static str {
    let suit = index / 13;
    let suitNameTable = [
        "Clubs", "Dimonds", "Hearts", "Spades", "Back"
    ];
    return suitNameTable[suit];
}


fn main() {
    let img = image::open("export_cards.png").expect("File not found!");
    let (width, height) = img.dimensions();
    let jsonfile = fs::read_to_string("export_cards.json").expect("Unable to read file");

    let spritedata: SpriteJson = serde_json::from_str(&jsonfile).expect("JSON was not well-formatted");
    let sprites: Vec<Sprite> = spritedata.texture_importer.sprite_sheet.sprites;

    let slen = sprites.len();
    let mut i = 0;
    for sprite in sprites {
        let sh = sprite.rect.height;
        let sw = sprite.rect.width;
        let sx = sprite.rect.x;
        let sy = height as i64 - sprite.rect.y - sprite.rect.height;
        let index = if i != 0 { i - 1 } else { slen - 1 };
        let value = indexToValue(index);
        let suit = indexToSuit(index);
        let card_name = getCardName(value, suit);

        println!("{i}, {card_name}");

        let subimg = img.view(sx as u32, sy as u32, sw as u32, sh as u32);
        subimg.to_image()
            .save_with_format(format!("{card_name}.png"), ImageFormat::Png)
            .unwrap();
        i+=1;
    }


}