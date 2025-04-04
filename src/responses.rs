use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AttachmentResource {
    pub id: String,
    pub type_: Option<String>,  // Note: Using type_ as type is a reserved keyword
    pub modelId: Option<String>,
    pub modelType: Option<String>,
    pub mimeType: Option<String>,
    pub filename: Option<String>,
    pub createdAt: chrono::DateTime<chrono::Utc>,
    pub updatedAt: chrono::DateTime<chrono::Utc>,
}


#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ChannelResource {
    pub id: String,
    pub name: String,
    pub sortOrder: i32,
    pub isDefault: bool,
    pub createdAt: chrono::DateTime<chrono::Utc>,
    pub updatedAt: chrono::DateTime<chrono::Utc>,
    pub deletedAt: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct MessageResource {
    pub id: String,
    pub userId: String,
    pub channelId: String,
    pub content: Option<String>,
    pub createdAt: chrono::DateTime<chrono::Utc>,
    pub updatedAt: chrono::DateTime<chrono::Utc>,
    pub deletedAt: Option<chrono::DateTime<chrono::Utc>>,
    pub deletedByUserId: Option<String>,
    pub user: UserResource,
    pub attachments: Vec<AttachmentResource>
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ServerInfoResource {
    pub id: String,
    pub name: String,
    pub description: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub channels: Vec<ChannelResource>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserResource {
    pub id: String,
    pub username: String,
    pub displayName: String,
    pub isSystemUser: bool,
    pub profilePicture: Option<String>,
    pub isOnline: bool,
    pub currentChannelId: Option<String>,
    pub connectedAt: Option<chrono::DateTime<chrono::Utc>>,
    pub createdAt: chrono::DateTime<chrono::Utc>,
    pub updatedAt: chrono::DateTime<chrono::Utc>,
}


#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ConnectionStateResource {
    pub isOnline: bool,
    pub connectedAt: Option<chrono::DateTime<chrono::Utc>>,
    pub currentChannelId: Option<String>,
    pub isAudioMuted: Option<bool>,
    pub isMicrophoneMuted: Option<bool>
}


#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserListResource {
    pub user: UserResource,
    pub connectionState: ConnectionStateResource,
}


#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AuthMeUserResource {
    pub id: String,
    pub username: String,
    pub displayName: String,
    pub isSystemUser: bool,
    pub profilePicture: Option<String>,
    pub isOnline: bool,
    pub currentChannelId: Option<String>,
    pub connectedAt: Option<chrono::DateTime<chrono::Utc>>,
    pub createdAt: chrono::DateTime<chrono::Utc>,
    pub updatedAt: chrono::DateTime<chrono::Utc>,
    pub permissions : Vec<String>,
}

