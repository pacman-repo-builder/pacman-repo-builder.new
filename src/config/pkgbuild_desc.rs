use super::PkgBuildName;
use serde::{Deserialize, Serialize};

/// Description of a single PKGBUILD directory.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum PkgBuildDesc {
    /// The PKGBUILD directory is in a local filesystem.
    Local(LocalPkgBuildDesc),
    /// The PKGBUILD directory is from a git repository.
    Git(GitPkgBuildDesc),
}

/// Description of a single local PKGBUILD directory.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct LocalPkgBuildDesc {
    /// Name(s) and base of the packages being built by the PKGBUILD.
    #[serde(flatten)]
    pub package: PkgBuildName,
    /// Location of the directory, relative to the configuration file.
    pub dir: String,
}

/// Description of a single remote PKGBUILD directory from a git repository.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct GitPkgBuildDesc {
    /// Name(s) and base of the packages being built by the PKGBUILD.
    #[serde(flatten)]
    pub package: PkgBuildName,
    /// URL of the repo to clone.
    pub git_url: String,
    /// Historical depth to clone.
    pub git_depth: Option<u64>,
    /// Branch, tag, or commit to check out.
    pub git_ref: Option<String>,
    /// Path to the sub directory that contains the PKGBUILD, relative to the git repo root.
    pub sub_dir: Option<String>,
}
