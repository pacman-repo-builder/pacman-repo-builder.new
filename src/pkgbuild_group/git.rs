use crate::{
    pkgbuild_desc::{GitPkgBuildDesc, PkgBuildDesc},
    pkgbuild_name::PkgBuildName,
};
use pipe_trait::Pipe;
use serde::{Deserialize, Serialize};

/// Grouping of multiple PKGBUILD git repositories with similar properties.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct GitPkgBuildGroup {
    /// Shared properties.
    #[serde(flatten)]
    pub header: GitPkgBuildHeader,
    /// Descriptors of member PKGBUILD git repositories.
    pub members: Vec<GitPkgBuildMember>,
}

/// Shared properties of members from within a [`GitPkgBuildGroup`].
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct GitPkgBuildHeader {
    /// Shared template of the git repository URLs.
    pub git_url_template: String,
    /// Default historical depth to clone.
    pub git_depth: Option<u64>,
    /// Shared template of branch names or tag names to check out.
    pub git_ref_template: Option<String>,
    /// Shared template of the path to the sub directory that contains the PKGBUILD, relative to the git repo root.
    pub sub_dir_template: Option<String>,
}

/// Member of a [`GitPkgBuildGroup`].
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum GitPkgBuildMember {
    /// The name of the package to be built.
    SimpleName(String),
    /// Complex specification with potential overrides.
    ComplexSpec(GitPkgBuildComplexMember),
}

/// Complex specification of a [`GitPkgBuildMember`] with potential overrides.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct GitPkgBuildComplexMember {
    /// Name(s) and base of the packages being built by the PKGBUILD.
    #[serde(flatten)]
    pub package: PkgBuildName,
    /// Override the URL of the git repository.
    pub git_url: Option<String>,
    /// Override historical depth to clone.
    pub git_depth: Option<u64>,
    /// Override the branch, tag, or commit to check out.
    pub git_ref: Option<String>,
    /// Override the path to the directory containing PKGBUILD, relative to the git repo root.
    pub sub_dir: Option<String>,
}

impl GitPkgBuildGroup {
    /// Append this group to a list as normalized descriptions.
    #[allow(unused)] // TODO: remove this
    pub(crate) fn normalize(self, target: &mut Vec<PkgBuildDesc>) {
        let GitPkgBuildGroup {
            header,
            members: packages,
        } = self;
        for member in packages {
            let desc = header.apply(member).pipe(PkgBuildDesc::Git);
            target.push(desc);
        }
    }
}

impl GitPkgBuildHeader {
    /// Apply this header to a member.
    fn apply(&self, member: GitPkgBuildMember) -> GitPkgBuildDesc {
        let member = member.normalize();
        GitPkgBuildDesc {
            package: member.package,
            git_url: member.git_url.unwrap_or_else(|| todo!()), // TODO: apply member.name.base to self.git_url_template
            git_depth: member.git_depth.or(self.git_depth),
            git_ref: member.git_ref.or(None), // TODO: apply member.name.base to self.git_ref_template
            sub_dir: member.sub_dir.or(None), // TODO: apply member.name.base to self.sub_dir_template
        }
    }
}

impl GitPkgBuildMember {
    /// Normalize all variants of this member into a complex spec.
    fn normalize(self) -> GitPkgBuildComplexMember {
        match self {
            Self::SimpleName(name) => GitPkgBuildComplexMember::with_single_name(name),
            Self::ComplexSpec(spec) => spec,
        }
    }
}

impl GitPkgBuildComplexMember {
    /// Create a complex spec with only a name.
    fn with_single_name(name: String) -> Self {
        GitPkgBuildComplexMember {
            package: PkgBuildName::single(name),
            git_url: None,
            git_depth: None,
            git_ref: None,
            sub_dir: None,
        }
    }
}
