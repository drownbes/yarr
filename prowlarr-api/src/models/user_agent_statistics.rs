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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAgentStatistics {
    #[serde(rename = "userAgent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<Option<String>>,
    #[serde(rename = "numberOfQueries", skip_serializing_if = "Option::is_none")]
    pub number_of_queries: Option<i32>,
    #[serde(rename = "numberOfGrabs", skip_serializing_if = "Option::is_none")]
    pub number_of_grabs: Option<i32>,
}

impl UserAgentStatistics {
    pub fn new() -> UserAgentStatistics {
        UserAgentStatistics {
            user_agent: None,
            number_of_queries: None,
            number_of_grabs: None,
        }
    }
}
