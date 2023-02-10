use std::fmt::{Debug, Formatter};
use std::{collections::HashMap, fmt};

use kira::sound::static_sound::StaticSoundData;
use kira::track::TrackHandle;
use rune::Any;
use serde::Deserialize;

use crate::render::data::{Model, RawFace, Vertex};
use crate::render::gui::GuiIds;
use crate::resource::function::Function;
use crate::resource::item::Item;
use crate::resource::model::Face;
use crate::resource::script::Script;
use crate::resource::tag::Tag;
use crate::resource::tile::{Tile, TileIds};
use crate::resource::translate::Translate;
use crate::util::id::{id_static, Id, IdRaw, Interner};

pub mod audio;
pub mod function;
pub mod item;
pub mod model;
pub mod script;
pub mod tag;
pub mod tile;
pub mod translate;

pub static JSON_EXT: &str = "json";
pub static OGG_EXT: &str = "ogg";
pub static RESOURCES_FOLDER: &str = "resources";

#[derive(Clone, Any)]
pub struct Registry {
    tiles: HashMap<Id, Tile>,
    scripts: HashMap<Id, Script>,
    tags: HashMap<Id, Tag>,
    items: HashMap<Id, Item>,

    #[rune(get, copy)]
    pub none: Id,
    #[rune(get, copy)]
    pub any: Id,
    #[rune(get, copy)]
    pub tile_ids: TileIds,
    #[rune(get, copy)]
    pub gui_ids: GuiIds,
}

impl Registry {
    pub fn get_tile(&self, id: Id) -> Option<Tile> {
        self.tiles.get(&id).cloned()
    }

    pub fn get_script(&self, id: Id) -> Option<Script> {
        self.scripts.get(&id).cloned()
    }

    pub fn get_tag(&self, id: Id) -> Option<Tag> {
        self.tags.get(&id).cloned()
    }

    pub fn get_item(&self, id: Id) -> Option<Item> {
        self.items.get(&id).cloned()
    }
}

pub struct ResourceManager {
    pub interner: Interner,
    pub track: TrackHandle,

    pub ordered_ids: Vec<Id>,

    pub registry: Registry,

    pub translates: Translate,
    pub functions: HashMap<Id, Function>,
    pub audio: HashMap<String, StaticSoundData>,
    pub faces: HashMap<Id, Face>,

    pub all_vertices: Vec<Vertex>,
    pub raw_models: HashMap<Id, Model>,
    pub raw_faces: Vec<RawFace>,
}

impl Debug for ResourceManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("<resource manager>")
    }
}

impl ResourceManager {
    pub fn new(track: TrackHandle) -> Self {
        let mut interner = Interner::new();
        let none = IdRaw::NONE.to_id(&mut interner);
        let any = id_static("automancy", "#any").to_id(&mut interner);
        let gui_ids = GuiIds::new(&mut interner);
        let tile_ids = TileIds::new(&mut interner);

        Self {
            interner,
            track,

            ordered_ids: vec![],

            registry: Registry {
                tiles: Default::default(),
                scripts: Default::default(),
                tags: Default::default(),
                items: Default::default(),

                none,
                any,
                tile_ids,
                gui_ids,
            },

            translates: Default::default(),
            functions: Default::default(),
            audio: Default::default(),
            faces: Default::default(),

            all_vertices: Default::default(),
            raw_models: Default::default(),
            raw_faces: Default::default(),
        }
    }
}