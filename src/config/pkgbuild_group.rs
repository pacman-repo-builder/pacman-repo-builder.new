pub mod git;
pub mod local;

pub use git::GitPkgBuildGroup;
pub use local::LocalPkgBuildGroup;

use super::PkgBuildDesc;
use serde::{Deserialize, Serialize};

/// Grouping of multiple PKGBUILD directories with similar properties.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum PkgBuildGroup {
    /// Not a group, but a single PKGBUILD directory.
    Single(PkgBuildDesc),
    /// Grouping of local directories.
    Local(LocalPkgBuildGroup),
    /// Grouping of git directories.
    Git(GitPkgBuildGroup),
}
