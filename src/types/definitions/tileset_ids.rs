use crate::prelude::*;
// To add a new tileset, add the name and id as pub const at the top
// This will allow access the tileset from Tilesets::get_by_[name/id]()

// Add the new name to the TILESET_XXX_NAME_TABLE it belongs to
// This will allow the tileset to be automatically loaded.

// Add the new id to the the TILESET_XXX_IDS_TABLE it belongs to
// This will cause loading to wait for this id to load.

// Tileset Names (Match filename to infile name)

/////////////////////////////////////////////////////////////////////
/// MISSING
/////////////////////////////////////////////////////////////////////
#[cfg(feature = "debug")] // Show TODO to catch bugs in debug mod
pub const TILESET_MISSING_NAME: &str = "missing_debug";
#[cfg(not(feature = "debug"))] // Show invisible for release builds
pub const TILESET_MISSING_NAME: &str = "missing";
pub const TILESET_MISSING_ID: TilesetId = 0;

/////////////////////////////////////////////////////////////////////
/// UI
/////////////////////////////////////////////////////////////////////
pub const TILESET_UI_NAME: &str = "ui";
pub const TILESET_UI_ID: TilesetId = 1;
#[rustfmt::skip] // pretty tables
pub const TILESET_UI_NAME_TABLE: &[&str] = &[
    TILESET_UI_NAME,
];
#[rustfmt::skip] // pretty tables
pub const TILESET_UI_IDS_TABLE: &[TilesetId] = &[
    TILESET_UI_ID,
];

/////////////////////////////////////////////////////////////////////
/// ACTORS
/////////////////////////////////////////////////////////////////////
pub const TILESET_ACTORS_NAME: &str = "actors";
pub const TILESET_ACTORS_ID: TilesetId = 2;

#[rustfmt::skip] // pretty tables
pub const TILESET_ACTORS_NAME_TABLE: &[&str] = &[
    TILESET_ACTORS_NAME,
];

#[rustfmt::skip] // pretty tables
pub const TILESET_ACTORS_IDS_TABLE: &[TilesetId] = &[
    TILESET_ACTORS_ID,
];

/////////////////////////////////////////////////////////////////////
/// ITEMS
/////////////////////////////////////////////////////////////////////
pub const TILESET_ITEMS_NAME: &str = "items";
pub const TILESET_ITEMS_ID: TilesetId = 3;

#[rustfmt::skip] // pretty tables
pub const TILESET_ITEMS_NAME_TABLE: &[&str] = &[
    TILESET_ITEMS_NAME,
];

#[rustfmt::skip] // pretty tables
pub const TILESET_ITEMS_IDS_TABLE: &[TilesetId] = &[
    TILESET_ITEMS_ID,
];

/////////////////////////////////////////////////////////////////////
/// FEATURES
/////////////////////////////////////////////////////////////////////
pub const TILESET_FEATURES_NAME: &str = "features";
pub const TILESET_FEATURES_ID: TilesetId = 4;

#[rustfmt::skip] // pretty tables
pub const TILESET_FEATURES_NAME_TABLE: &[&str] = &[
    TILESET_FEATURES_NAME,
];

#[rustfmt::skip] // pretty tables
pub const TILESET_FEATURES_IDS_TABLE: &[TilesetId] = &[
    TILESET_FEATURES_ID,
];

/////////////////////////////////////////////////////////////////////
/// TERRAIN
/////////////////////////////////////////////////////////////////////
pub const TILESET_TERRAIN_NAME: &str = "terrain";
pub const TILESET_TERRAIN_ID: TilesetId = 5;

#[rustfmt::skip] // pretty tables
pub const TILESET_TERRAIN_NAME_TABLE: &[&str] = &[
    TILESET_TERRAIN_NAME,
];

#[rustfmt::skip] // pretty tables
pub const TILESET_TERRAIN_IDS_TABLE: &[TilesetId] = &[
    TILESET_TERRAIN_ID,
];
/////////////////////////////////////////////////////////////////////
/// FUNCTIONS
/////////////////////////////////////////////////////////////////////

/// This function is used during tileset loading
pub fn get_tileset_paths() -> Vec<String> {
    let mut ret = Vec::new();

    // add the "missing" tileset
    ret.push(format!(
        "{DEFINITIONS_FOLDER}{TILESET_MISSING_NAME}{RON_EXT}"
    ));

    // add the "ui" tilesets
    for name in TILESET_UI_NAME_TABLE {
        ret.push(format!("{DEFINITIONS_FOLDER}{name}{RON_EXT}"));
    }

    // add the "actors" tilesets
    for name in TILESET_ACTORS_NAME_TABLE {
        ret.push(format!("{DEFINITIONS_FOLDER}{name}{RON_EXT}"));
    }

    // add the "items" tilesets
    for name in TILESET_ITEMS_NAME_TABLE {
        ret.push(format!("{DEFINITIONS_FOLDER}{name}{RON_EXT}"));
    }

    // add the "features" tilesets
    for name in TILESET_FEATURES_NAME_TABLE {
        ret.push(format!("{DEFINITIONS_FOLDER}{name}{RON_EXT}"));
    }

    // add the "terrain" tilesets
    for name in TILESET_TERRAIN_NAME_TABLE {
        ret.push(format!("{DEFINITIONS_FOLDER}{name}{RON_EXT}"));
    }

    ret
}

/////////////////////////////////////////////////////////////////////
/// INTERNAL FILE STRUCTURE
/////////////////////////////////////////////////////////////////////

// Tileset Extension
const RON_EXT: &str = ".ron";

// Tileset definitions folder
const DEFINITIONS_FOLDER: &str = "images/tilesets/definitions/";
