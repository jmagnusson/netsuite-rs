use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use log::LevelFilter;

use super::{env::EnvVar, CliError};
use crate::metadata::VERSION;
use crate::params::ParamStr;

impl FromStr for ParamStr {
    type Err = CliError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s
            .split_once('=')
            .map(|(k, v)| ParamStr::new(k.trim().to_owned(), v.trim().to_owned()));
        parsed.ok_or(CliError::BadParam)
    }
}

#[derive(Debug, Parser)]
#[clap(name = "netsuite", version = VERSION)]
pub(crate) struct Opts {
    #[clap(short = 's', long, env, default_value = "netsuite")]
    ini_section: String,
    /// Where to load INI from, defaults to your OS's config directory.
    #[clap(short = 'p', long, env)]
    ini_path: Option<PathBuf>,
    #[clap(subcommand)]
    pub(crate) subcmd: SubCommand,
    /// Set the log level
    #[clap(
        short = 'l',
        long = "log-level",
        value_name = "level",
        env = "LOG_LEVEL"
    )]
    level_filter: Option<LevelFilter>,
}

#[derive(Debug, Parser)]
pub(crate) enum SubCommand {
    #[clap(name = "rest-api")]
    RestApiOpts {
        #[clap(short, long, env = EnvVar::Account.into())]
        account: String,
        #[clap(short = 'c', long, env = EnvVar::ConsumerKey.into())]
        consumer_key: String,
        #[clap(short = 'C', long, env = EnvVar::ConsumerSecret.into())]
        consumer_secret: String,
        #[clap(short = 't', long, env = EnvVar::TokenId.into())]
        token_id: String,
        #[clap(short = 'T', long, env = EnvVar::TokenSecret.into())]
        token_secret: String,
        #[clap(subcommand)]
        subcmd: RestApiSubCommand,
    },
    #[clap(name = "default-ini-path")]
    DefaultIniPath,
}

#[derive(Debug, Parser)]
pub(crate) enum RestApiSubCommand {
    #[clap(name = "get")]
    Get {
        /// The endpoint to get data for
        endpoint: String,
        #[clap(short = 'p', long = "param")]
        params: Vec<ParamStr>,
        #[clap(short = 'H', long = "header")]
        headers: Vec<ParamStr>,
    },
    #[clap(name = "post")]
    Post {
        /// The endpoint to submit data to
        endpoint: String,
        /// A file containing data to submit
        file: Option<PathBuf>,
        #[clap(short = 'p', long = "param")]
        params: Vec<ParamStr>,
        #[clap(short = 'H', long = "header")]
        headers: Vec<ParamStr>,
    },
    #[clap(name = "put")]
    Put {
        /// The endpoint to submit data to
        endpoint: String,
        /// A file containing data to submit
        file: Option<PathBuf>,
        #[clap(short = 'p', long = "param")]
        params: Vec<ParamStr>,
        #[clap(short = 'H', long = "header")]
        headers: Vec<ParamStr>,
    },
    #[clap(name = "patch")]
    Patch {
        /// The resource to update
        endpoint: String,
        /// A file containing the update data
        file: Option<PathBuf>,
        #[clap(short = 'p', long = "param")]
        params: Vec<ParamStr>,
        #[clap(short = 'H', long = "header")]
        headers: Vec<ParamStr>,
    },
    #[clap(name = "delete")]
    Delete {
        /// The resource to delete
        endpoint: String,
        #[clap(short = 'p', long = "param")]
        params: Vec<ParamStr>,
        #[clap(short = 'H', long = "header")]
        headers: Vec<ParamStr>,
    },
    #[clap(name = "suiteql")]
    SuiteQl {
        /// The query to execute. If `-` is provided, query will be read from standard input.
        query: String,
        #[clap(short, long, default_value = "1000")]
        limit: usize,
        #[clap(short, long, default_value = "0")]
        offset: usize,
    },
}
