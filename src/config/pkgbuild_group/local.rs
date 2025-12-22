use crate::config::{pkgbuild_desc::LocalPkgBuildDesc, PkgBuildDesc, PkgBuildName};
use pipe_trait::Pipe;
use serde::{Deserialize, Serialize};

/// Grouping of multiple local PKGBUILD directories with similar properties.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct LocalPkgBuildGroup {
    /// Shared properties.
    #[serde(flatten)]
    pub header: LocalPkgBuildHeader,
    /// Descriptors of member PKGBUILD directories.
    pub members: Vec<LocalPkgBuildMember>,
}

/// Shared properties of members from within a [`LocalPkgBuildGroup`].
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct LocalPkgBuildHeader {
    /// Shared template of local directory paths, relative to the configuration file.
    pub dir_path_template: String,
}

/// Member of a [`LocalPkgBuildGroup`].
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum LocalPkgBuildMember {
    /// The name of the package to be built.
    SimpleName(String),
    /// Complex specification with potential overrides.
    ComplexSpec(LocalPkgBuildComplexMember),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct LocalPkgBuildComplexMember {
    /// Name(s) and base of the packages being built by the PKGBUILD.
    #[serde(flatten)]
    pub package: PkgBuildName,
}

impl LocalPkgBuildGroup {
    /// Append this group to a list as normalized descriptions.
    #[allow(unused)] // TODO: remove this
    pub(crate) fn normalize(self, target: &mut Vec<PkgBuildDesc>) {
        let LocalPkgBuildGroup {
            header,
            members: packages,
        } = self;
        for member in packages {
            let desc = header.apply(member).pipe(PkgBuildDesc::Local);
            target.push(desc);
        }
    }
}

impl LocalPkgBuildHeader {
    /// Apply this header to a member.
    fn apply(&self, member: LocalPkgBuildMember) -> LocalPkgBuildDesc {
        let package = match member {
            LocalPkgBuildMember::SimpleName(name) => PkgBuildName::single(name),
            LocalPkgBuildMember::ComplexSpec(LocalPkgBuildComplexMember { package: name }) => name,
        };
        LocalPkgBuildDesc {
            package,
            dir: "TODO".to_string(), // TODO: apply package.base to self.dir_path_template
        }
    }
}
