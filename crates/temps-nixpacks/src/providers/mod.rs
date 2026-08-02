use crate::nixpacks::{app::App, environment::Environment, plan::BuildPlan};
use anyhow::Result;

pub mod clojure;
pub mod cobol;
pub mod crystal;
pub mod csharp;
pub mod dart;
pub mod deno;
pub mod elixir;
pub mod fsharp;
pub mod gleam;
pub mod go;
pub mod haskell;
pub mod java;
pub mod lunatic;
pub mod node;
pub mod php;
pub mod procfile;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod scala;
pub mod scheme;
pub mod staticfile;
pub mod swift;
pub mod zig;

pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    fn detect(&self, _app: &App, _env: &Environment) -> Result<bool> {
        Ok(false)
    }
    fn get_build_plan(&self, _app: &App, _environment: &Environment) -> Result<Option<BuildPlan>>;
}
