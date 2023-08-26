use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use serde::*;
use crate::structs::*;

use crate::{
    constants::*, 
    enums::*
};

#[derive(Resource, Default)]
pub struct FontResource {
    pub font: Handle<Font>,
    pub preview_font: TextStyle,
    pub p1_font: TextStyle,
}

#[derive(Resource)]
pub struct UIMenu {
    pub ui: Entity
}

#[derive(Resource)]
pub struct SubBuilderPreview {
    pub entity: Entity,
    pub part: SubPart,
    pub rotation: PieceRotation
}

#[derive(Resource)]
pub struct LocalPlayerHandle(pub usize);

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "bullet.png")]
    pub bullet: Handle<Image>,
    #[asset(path = "explosion.png")]
    pub explosion: Handle<Image>,
}

#[derive(Resource)]
pub struct Colors {
    pub normal_text: Color,
    pub submarine_text: Color,
    pub menu_background: Color,
    pub node_background: Color,
    pub button_normal: Color,
    pub button_pressed: Color
}

#[derive(Resource, Serialize, Deserialize)]
pub struct Submarine {
    pub parts: Vec<Vec<char>>,
    pub rotations: Vec<Vec<PieceRotation>>
}

impl Default for Submarine {
    fn default() -> Self {
        Submarine{ parts: Vec::new(), rotations: Vec::new() }
    }
}

#[derive(Resource)]
pub struct SubBuilder {
    pub root: Entity,
    pub parts: Vec<Vec<Option<Entity>>>
}

impl Default for SubBuilder {
    fn default() -> Self {
        SubBuilder { root: Entity::PLACEHOLDER, parts: Vec::new() }
    }
}
