use serde::{Deserialize, Serialize};

/// Name(s) and base of the packages being built by a PKGBUILD.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PkgBuildName {
    /// The PKGBUILD builds only a single package.
    Single(PkgBuildSingleName),
    /// The PKGBUILD builds multiple packages.
    Split(PkgBuildSplitName),
}

impl PkgBuildName {
    /// Create the name of a single-package PKGBUILD.
    pub fn single(name: String) -> Self {
        PkgBuildName::Single(PkgBuildSingleName { name })
    }

    /// Create the name of a split-packages PKGBUILD.
    pub fn split(base: String, names: Vec<String>) -> Self {
        PkgBuildName::Split(PkgBuildSplitName { base, names })
    }
}

/// Name of the only package being built by a single-package PKGBUILD.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct PkgBuildSingleName {
    /// Name of the package.
    pub name: String,
}

/// Names and base of the packages being built by a split-packages PKGBUILD.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct PkgBuildSplitName {
    /// Base of the PKGBUILD.
    pub base: String,
    /// Names of the packages.
    pub names: Vec<String>,
}
