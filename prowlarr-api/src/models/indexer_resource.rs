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
pub struct IndexerResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "fields", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Option<Vec<models::Field>>>,
    #[serde(rename = "implementationName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub implementation_name: Option<Option<String>>,
    #[serde(rename = "implementation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub implementation: Option<Option<String>>,
    #[serde(rename = "configContract", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub config_contract: Option<Option<String>>,
    #[serde(rename = "infoLink", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub info_link: Option<Option<String>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<models::ProviderMessage>>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<i32>>>,
    #[serde(rename = "presets", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub presets: Option<Option<Vec<models::IndexerResource>>>,
    #[serde(rename = "indexerUrls", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub indexer_urls: Option<Option<Vec<String>>>,
    #[serde(rename = "legacyUrls", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub legacy_urls: Option<Option<Vec<String>>>,
    #[serde(rename = "definitionName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub definition_name: Option<Option<String>>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "language", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub language: Option<Option<String>>,
    #[serde(rename = "encoding", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Option<String>>,
    #[serde(rename = "enable", skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(rename = "redirect", skip_serializing_if = "Option::is_none")]
    pub redirect: Option<bool>,
    #[serde(rename = "supportsRss", skip_serializing_if = "Option::is_none")]
    pub supports_rss: Option<bool>,
    #[serde(rename = "supportsSearch", skip_serializing_if = "Option::is_none")]
    pub supports_search: Option<bool>,
    #[serde(rename = "supportsRedirect", skip_serializing_if = "Option::is_none")]
    pub supports_redirect: Option<bool>,
    #[serde(rename = "supportsPagination", skip_serializing_if = "Option::is_none")]
    pub supports_pagination: Option<bool>,
    #[serde(rename = "appProfileId", skip_serializing_if = "Option::is_none")]
    pub app_profile_id: Option<i32>,
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<models::DownloadProtocol>,
    #[serde(rename = "privacy", skip_serializing_if = "Option::is_none")]
    pub privacy: Option<models::IndexerPrivacy>,
    #[serde(rename = "capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Box<models::IndexerCapabilityResource>>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "downloadClientId", skip_serializing_if = "Option::is_none")]
    pub download_client_id: Option<i32>,
    #[serde(rename = "added", skip_serializing_if = "Option::is_none")]
    pub added: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<models::IndexerStatusResource>>,
    #[serde(rename = "sortName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sort_name: Option<Option<String>>,
}

impl IndexerResource {
    pub fn new() -> IndexerResource {
        IndexerResource {
            id: None,
            name: None,
            fields: None,
            implementation_name: None,
            implementation: None,
            config_contract: None,
            info_link: None,
            message: None,
            tags: None,
            presets: None,
            indexer_urls: None,
            legacy_urls: None,
            definition_name: None,
            description: None,
            language: None,
            encoding: None,
            enable: None,
            redirect: None,
            supports_rss: None,
            supports_search: None,
            supports_redirect: None,
            supports_pagination: None,
            app_profile_id: None,
            protocol: None,
            privacy: None,
            capabilities: None,
            priority: None,
            download_client_id: None,
            added: None,
            status: None,
            sort_name: None,
        }
    }
}
