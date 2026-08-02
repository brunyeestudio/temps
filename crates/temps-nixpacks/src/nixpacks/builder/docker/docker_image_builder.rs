use super::{
    dockerfile_generation::{DockerfileGenerator, OutputDir},
    DockerBuilderOptions,
};
use crate::nixpacks::{environment::Environment, plan::BuildPlan};
use anyhow::{bail, Context, Result};
use std::{fs, fs::File};

/// Writes a Dockerfile (and supporting Nix/assets files) for a build plan.
///
/// Unlike upstream nixpacks, this never invokes `docker build` — Temps builds
/// images itself after reading the generated Dockerfile.
pub struct DockerImageBuilder {
    options: DockerBuilderOptions,
}

/// Determine where to write generated assets like Dockerfiles.
///
/// Temps always passes `out_dir`; the previous tempdir fallback is intentionally
/// not supported.
fn get_output_dir(options: &DockerBuilderOptions) -> Result<OutputDir> {
    if let Some(value) = &options.out_dir {
        OutputDir::new(value.into())
    } else {
        bail!(
            "out_dir is required: temps-nixpacks only writes Dockerfiles to a provided output directory"
        )
    }
}

impl DockerImageBuilder {
    pub fn new(options: DockerBuilderOptions) -> DockerImageBuilder {
        DockerImageBuilder { options }
    }

    /// Generate a Dockerfile from a BuildPlan and write it under `out_dir/.nixpacks/`.
    pub fn write_dockerfile(&self, plan: &BuildPlan, env: &Environment) -> Result<()> {
        let output = get_output_dir(&self.options)?;
        output.ensure_output_exists()?;

        let dockerfile = plan
            .generate_dockerfile(&self.options, env, &output)
            .context("Generating Dockerfile for plan")?;

        self.write_dockerfile_file(dockerfile, &output)
            .context("Writing Dockerfile")?;
        plan.write_supporting_files(&self.options, env, &output)
            .context("Writing supporting files")?;

        Ok(())
    }

    /// Writes the generated Dockerfile to the output dir.
    fn write_dockerfile_file(&self, dockerfile: String, output: &OutputDir) -> Result<()> {
        let dockerfile_path = output.get_absolute_path("Dockerfile");
        File::create(dockerfile_path.clone()).context("Creating Dockerfile file")?;
        fs::write(dockerfile_path, dockerfile).context("Write Dockerfile")?;

        Ok(())
    }
}
