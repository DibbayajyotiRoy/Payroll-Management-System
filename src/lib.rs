#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;

pub mod db;
pub mod domain;
pub mod calculation;
pub mod services;
pub mod config;
pub mod api;
pub mod web;
pub mod schema;