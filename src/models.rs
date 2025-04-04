use crate::responses::{AuthMeUserResource, ChannelResource, MessageResource, UserResource};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub sort_order: i32,
    pub is_default: i8,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Channel {
    pub fn to_resource(&self) -> ChannelResource {
        return ChannelResource {
            id: self.id.to_owned(),
            name: self.name.to_owned(),
            sortOrder: self.sort_order.to_owned(),
            isDefault: self.is_default != 0,
            createdAt: self.created_at.to_owned(),
            updatedAt: self.updated_at.to_owned(),
            deletedAt: self.deleted_at.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Message {
    pub id: String,
    pub user_id: String,
    pub channel_id: String,
    pub content: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub deleted_by_user_id: Option<String>,
}

impl Message {
    pub fn to_resource(&self, user: UserResource) -> MessageResource {
        MessageResource {
            id: self.id.to_owned(),
            userId: self.user_id.to_owned(),
            channelId: self.channel_id.to_owned(),
            content: self.content.to_owned(),
            createdAt: self.created_at.to_owned(),
            updatedAt: self.updated_at.to_owned(),
            deletedAt: self.deleted_at.to_owned(),
            deletedByUserId: self.deleted_by_user_id.to_owned(),
            user,
            attachments: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub password: String,
    pub is_system_user: i8,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn to_resource(&self) -> UserResource {
        return UserResource {
            id: self.id.to_owned(),
            username: self.username.to_owned(),
            displayName: self.display_name.to_owned(),
            isSystemUser: self.is_system_user == 1,
            profilePicture: None,
            isOnline: false,
            currentChannelId: None,
            connectedAt: None,
            createdAt: self.created_at.to_owned(),
            updatedAt: self.updated_at.to_owned(),
        }
    }

    pub fn to_auth_me_resource(&self) -> AuthMeUserResource {
        return AuthMeUserResource {
            id: self.id.to_owned(),
            username: self.username.to_owned(),
            displayName: self.display_name.to_owned(),
            isSystemUser: self.is_system_user == 1,
            profilePicture: None,
            isOnline: false,
            currentChannelId: None,
            connectedAt: None,
            createdAt: self.created_at.to_owned(),
            updatedAt: self.updated_at.to_owned(),
            permissions: vec![],
        }
    }
}
