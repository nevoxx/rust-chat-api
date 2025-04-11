pub mod socket_listen_events {
    pub const CONNECTION: &str = "connection";
    pub const SEND_CHAT_MESSAGE: &str = "sendChatMessage";
    pub const DISCONNECT: &str = "disconnect";
    pub const CONNECT_ERROR: &str = "connect_error";
    pub const SEND_POKE: &str = "sendPoke";
    pub const SEND_KICK: &str = "sendKick";
    pub const SEND_USER_IS_TYPING: &str = "sendUserIsTyping";
    pub const SEND_USER_AUDIO_MUTE_STATUS_CHANGED: &str = "sendUserAudioMuteStatusChanged";
    pub const SEND_USER_MICROPHONE_STATUS_CHANGED: &str = "sendUserMicrophoneStatusChanged";
}

pub mod socket_publish_events {
    pub const RECEIVE_CHAT_MESSAGE: &str = "receiveChatMessage";
    pub const RECEIVE_POKE: &str = "receivePoke";
    pub const RECEIVE_KICK: &str = "receiveKick";
    pub const UPDATE_USER: &str = "updateUser";
    pub const UPDATE_MESSAGE: &str = "updateMessage";
    pub const UPDATE_CHANNELS: &str = "updateChannels";
    pub const RECEIVE_USER_IS_TYPING: &str = "receiveUserIsTyping";
    pub const RECEIVE_USER_AUDIO_MUTE_STATUS_CHANGED: &str = "receiveUserAudioMuteStatusChanged";
    pub const RECEIVE_USER_MICROPHONE_STATUS_CHANGED: &str = "receiveUserMicrophoneStatusChanged";
}
