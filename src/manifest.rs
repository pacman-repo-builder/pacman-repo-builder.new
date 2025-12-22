use crate::{file_base_name::FileBaseName, pkgbuild_group::PkgBuildGroup, repo_name::RepoName};
use serde::{Deserialize, Serialize};

/// Data stored in a manifest file.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub struct Manifest {
    /// The name of or path to the CLI program to build images and run containers.
    ///
    /// This program reads from a [manifest file](Manifest::container_file).
    ///
    /// Examples: `docker`, `podman`.
    pub container_manager: String,

    /// The name of the manifest file to be read by the [container manager](Manifest::container_manager).
    ///
    /// This name must be a base name, not a path.
    ///
    /// Examples: `Dockerfile`, `Containerfile`.
    pub container_file: FileBaseName,

    /// Directory to store all the repositories that contain PKGBUILD and .SRCINFO.
    ///
    /// The path is relative to the manifest file.
    pub pkgbuild_dir: Option<String>,

    /// Directory to store all the directories to build container images in.
    ///
    /// The path is relative to the manifest file.
    pub container_dir: Option<String>,

    /// Directory to store all the built pacman package archives.
    ///
    /// The path is relative to the manifest file.
    pub package_dir: Option<String>,

    /// Name of the repository of the built pacman packages.
    pub repo_name: RepoName,

    /// Sources from whence to fetch PKGBUILD and .SRCINFO to build packages.
    pub sources: Vec<PkgBuildGroup>,
}
