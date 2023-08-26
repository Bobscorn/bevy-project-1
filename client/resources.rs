use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{constants::*, enums::PieceRotation};

pub struct SubPiece {
    pub symbol: char,
    pub label: &'static str,
    pub description: &'static str 
}

#[derive(Resource)]
pub struct UIMenu {
    pub ui: Entity
}

#[derive(Resource)]
pub struct SubBuilderPreview {
    pub ent: Entity,
    pub piece: &'static SubPiece,
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

#[derive(Resource)]
pub struct Submarine {
    pub pieces: [[char; SUB_MAX_HEIGHT]; SUB_MAX_WIDTH],
    pub rotations: [[PieceRotation; SUB_MAX_HEIGHT]; SUB_MAX_WIDTH]
}

impl Default for Submarine {
    fn default() -> Self {
        Submarine{ pieces: [[EMPTY_CHAR; SUB_MAX_HEIGHT]; SUB_MAX_WIDTH], rotations: [[PieceRotation::North; SUB_MAX_HEIGHT]; SUB_MAX_WIDTH] }
    }
}

#[derive(Resource)]
pub struct SubBuilder {
    pub root: Entity,
    pub pieces: [[Option<Entity>; SUB_MAX_HEIGHT]; SUB_MAX_WIDTH]
}

impl Default for SubBuilder {
    fn default() -> Self {
        SubBuilder { root: Entity::PLACEHOLDER, pieces: [[None; SUB_MAX_HEIGHT]; SUB_MAX_WIDTH] }
    }
}
