pub mod file_base_name;

pub use file_base_name::FileBaseName;

use serde::{Deserialize, Serialize};

/// Data stored in a configuration file.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// The name of or path to the CLI program to build images and run containers.
    ///
    /// This program reads from a [manifest file](Config::container_file).
    ///
    /// Examples: `docker`, `podman`.
    pub container_manager: String,

    /// The name of the manifest file to be read by the [container manager](Config::container_manager).
    ///
    /// This name must be a base name, not a path.
    ///
    /// Examples: `Dockerfile`, `Containerfile`.
    pub container_file: FileBaseName,

    /// Directory to store all the repositories that contain PKGBUILD and .SRCINFO.
    ///
    /// The path is relative to the configuration file.
    pub pkgbuild_dir: Option<String>,

    /// Directory to store all the directories to build container images in.
    ///
    /// The path is relative to the configuration file.
    pub container_dir: Option<String>,
}
