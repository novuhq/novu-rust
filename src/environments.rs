use serde::{Deserialize, Serialize};

/** Environment api response
 * {
  "data": {
    "_id": "string",
    "name": "string",
    "_organizationId": "string",
    "identifier": "string",
    "apiKeys": [
      {
        "key": "string",
        "_userId": "string"
      }
    ],
    "_parentId": "string"
  }
}
 */
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    pub _id: String,
    pub name: String,
    pub _organization_id: String,
    pub identifier: String,
    pub api_keys: Vec<ApiKey>,
    pub _parent_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    pub key: String,
    pub _user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentPayload {
    pub name: String,
    pub parent_id: Option<String>,
}

/*
{
    'name' => 'Local', # optional
    'identifier' => 'local', # optional
    'parentId' => '7789', # optional
    'dns' => { # optional
        'inboundParseDomain' => 'dev.test' # optional
    }
}

*/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEnvironmentPayload {
    pub name: Option<String>,
    pub identifier: Option<String>,
    pub parent_id: Option<String>,
    pub dns: Option<Dns>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dns {
    pub inbound_parse_domain: Option<String>,
}
