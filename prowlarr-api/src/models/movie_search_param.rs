/*
 * Prowlarr
 *
 * Prowlarr API docs
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MovieSearchParam {
    #[serde(rename = "q")]
    Q,
    #[serde(rename = "imdbId")]
    ImdbId,
    #[serde(rename = "tmdbId")]
    TmdbId,
    #[serde(rename = "imdbTitle")]
    ImdbTitle,
    #[serde(rename = "imdbYear")]
    ImdbYear,
    #[serde(rename = "traktId")]
    TraktId,
    #[serde(rename = "genre")]
    Genre,
    #[serde(rename = "doubanId")]
    DoubanId,
    #[serde(rename = "year")]
    Year,

}

impl std::fmt::Display for MovieSearchParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Q => write!(f, "q"),
            Self::ImdbId => write!(f, "imdbId"),
            Self::TmdbId => write!(f, "tmdbId"),
            Self::ImdbTitle => write!(f, "imdbTitle"),
            Self::ImdbYear => write!(f, "imdbYear"),
            Self::TraktId => write!(f, "traktId"),
            Self::Genre => write!(f, "genre"),
            Self::DoubanId => write!(f, "doubanId"),
            Self::Year => write!(f, "year"),
        }
    }
}

impl Default for MovieSearchParam {
    fn default() -> MovieSearchParam {
        Self::Q
    }
}
