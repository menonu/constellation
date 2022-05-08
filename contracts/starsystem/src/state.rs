use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint64};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub name: String,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum SunClass {
    SuperGiant,
    BrightGiant,
    Giant,
    SubGiant,
    MainSequence,
    SubDwarf,
    WhiteDwarf,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Sun {
    pub name: String,
    pub sun_class: SunClass,
    pub luminosity: Uint64,
}

pub const SUN: Item<Sun> = Item::new("sun");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum PlanetType {
    Rock,
    Ice,
    Gas,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Planet {
    pub name: String,
    pub planet_type: PlanetType,
    pub size: Uint64,
    pub satellites: Option<()>,
}

pub type Planets = Vec<Planet>;

pub const PLANETS: Item<Planets> = Item::new("planets");
