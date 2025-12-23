use derive_more::{Display, Error};
use pipe_trait::Pipe;

/// This trait carries the common types used by all other query traits.
pub trait QueryCommon {
    /// Value to return by every query.
    type Value;
}

/// Query information from a PKGBUILD entry.
pub trait QueryPkgbuild: QueryCommon {
    /// Query the `base` value in the manifest corresponding to the PKGBUILD entry.
    fn base(&self) -> Self::Value;
}

/// Query the environment variables.
pub trait QueryEnv: QueryCommon {
    /// Name of the environment variable.
    type Name;
    /// Error when the query fails.
    type Error;
    /// Query the environment variable.
    fn env(&self, name: Self::Name) -> Result<Option<Self::Value>, Self::Error>;
}

/// Run a command and get its stdout as [value](QueryCommon::Value).
pub trait QueryCmd: QueryCommon {
    /// Command to run.
    type Command;
    /// Error when the command fails.
    type Error;
    /// Run a command to get its stdout.
    fn run(&self, command: Self::Command) -> Result<Self::Value, Self::Error>;
}

/// Run query for a template parameter.
pub trait QueryTemplateParam: QueryPkgbuild + QueryEnv + QueryCmd {
    /// Query a value for a template parameter
    fn query(&self, query: QueryTemplateParamInput<Self>) -> QueryTemplateParamResult<Self> {
        match query {
            TemplateParamQuery::PkgbuildBase => self.base().pipe(Ok),
            TemplateParamQuery::GetEnv(name) => self
                .env(name)
                .map_err(QueryTemplateParamError::Env)?
                .ok_or(QueryTemplateParamError::NoEnv),
            TemplateParamQuery::RunCommand(command) => {
                self.run(command).map_err(QueryTemplateParamError::Command)
            }
        }
    }
}

impl<Params: QueryPkgbuild + QueryEnv + QueryCmd> QueryTemplateParam for Params {}

/// Query to pass into [`QueryTemplateParam::query`].
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum TemplateParamQuery<EnvName, Command> {
    /// Query the base name that corresponds to the concerning PKGBUILD.
    PkgbuildBase,
    /// Query an environment variable.
    GetEnv(EnvName),
    /// Run a command and get its stdout.
    RunCommand(Command),
}

/// Error when parsing [`TemplateParamQuery`] fails.
#[derive(Debug, Display, Clone, Error)]
#[non_exhaustive]
pub enum ParseTemplateParamQueryError<'a> {
    #[display("Expecting a colon but found EOI")]
    MissingColon,
    #[display("Unknown scheme: {_0:?}")]
    UnknownScheme(#[error(not(source))] &'a str),
}

impl<'a> TemplateParamQuery<&'a str, &'a str> {
    /// Parse a raw query string.
    pub fn parse(raw_query: &'a str) -> Result<Self, ParseTemplateParamQueryError<'a>> {
        let raw_query = raw_query.trim();
        if raw_query == "base" {
            return Ok(TemplateParamQuery::PkgbuildBase);
        }

        let (scheme, suffix) = raw_query
            .split_once(':')
            .ok_or(ParseTemplateParamQueryError::MissingColon)?;
        let suffix = suffix.trim_start();

        Ok(match scheme {
            "env" => TemplateParamQuery::GetEnv(suffix),
            "cmd" => TemplateParamQuery::RunCommand(suffix),
            _ => return Err(ParseTemplateParamQueryError::UnknownScheme(scheme)),
        })
    }
}

impl<'a> TryFrom<&'a str> for TemplateParamQuery<&'a str, &'a str> {
    type Error = ParseTemplateParamQueryError<'a>;
    fn try_from(raw_query: &'a str) -> Result<Self, Self::Error> {
        TemplateParamQuery::parse(raw_query)
    }
}

/// Error type of [`QueryTemplateParam::query`] fails.
#[derive(Debug, Display, Clone, Error)]
#[non_exhaustive]
pub enum QueryTemplateParamError<EnvError, CommandError> {
    /// No env with the given name.
    #[display("No env with the given name")]
    NoEnv,
    /// Failed to query an environment variable.
    Env(EnvError),
    /// Failed to run a command.
    Command(CommandError),
}

/// Input to pass to [`QueryTemplateParam::query`].
type QueryTemplateParamInput<Params> =
    TemplateParamQuery<<Params as QueryEnv>::Name, <Params as QueryCmd>::Command>;

/// Return type of [`QueryTemplateParam::query`].
type QueryTemplateParamResult<Params> = Result<
    <Params as QueryCommon>::Value,
    QueryTemplateParamError<<Params as QueryEnv>::Error, <Params as QueryCmd>::Error>,
>;
