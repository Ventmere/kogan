use std::str::FromStr;

use error::KoganError;

pub mod client;
pub mod error;
pub mod product;
pub mod category;

#[derive(Debug)]
pub enum KoganEnv {
    Test,
    Live,
}

impl FromStr for KoganEnv {
    type Err = error::KoganError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Test" => Ok(Self::Test),
            "Live" => Ok(Self::Live),
            other => Err(KoganError::InvalidKoganEnvString(other.to_owned()))
        }
    }
}

impl KoganEnv {
    pub fn shopfront_url(&self) -> &'static str {
        match *self {
            KoganEnv::Test => "https://www-marketplace.aws.kgn.io/au",
            KoganEnv::Live => "https://www.kogan.com/au",
        }
    }

    pub fn base_api_url(&self) -> &'static str {
        match *self {
            KoganEnv::Test => "https://nimda-marketplace.aws.kgn.io/api/marketplace/v2",
            KoganEnv::Live => "https://nimda.kogan.com/api/marketplace/v2",
        }
    }
}
