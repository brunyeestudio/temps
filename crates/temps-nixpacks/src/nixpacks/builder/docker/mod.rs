/// Options for generating a Dockerfile.
///
/// Temps only uses this crate to write Dockerfiles (never to run `docker build`),
/// so CLI/runtime docker flags from upstream nixpacks are intentionally omitted.
#[derive(Clone, Default, Debug)]
pub struct DockerBuilderOptions {
    /// Directory where `.nixpacks/Dockerfile` and supporting files are written.
    pub out_dir: Option<String>,
    /// Optional BuildKit cache key prefix for phase `RUN --mount=type=cache`.
    pub cache_key: Option<String>,
    /// Disable BuildKit cache mounts in generated Dockerfiles.
    pub no_cache: bool,
}

mod cache;
pub mod docker_image_builder;
mod dockerfile_generation;
pub mod utils;
