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
pub enum HistoryEventType {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "releaseGrabbed")]
    ReleaseGrabbed,
    #[serde(rename = "indexerQuery")]
    IndexerQuery,
    #[serde(rename = "indexerRss")]
    IndexerRss,
    #[serde(rename = "indexerAuth")]
    IndexerAuth,
    #[serde(rename = "indexerInfo")]
    IndexerInfo,

}

impl std::fmt::Display for HistoryEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::ReleaseGrabbed => write!(f, "releaseGrabbed"),
            Self::IndexerQuery => write!(f, "indexerQuery"),
            Self::IndexerRss => write!(f, "indexerRss"),
            Self::IndexerAuth => write!(f, "indexerAuth"),
            Self::IndexerInfo => write!(f, "indexerInfo"),
        }
    }
}

impl Default for HistoryEventType {
    fn default() -> HistoryEventType {
        Self::Unknown
    }
}
