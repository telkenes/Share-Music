/*
 *  Copyright (c) 2021-2022 tooboredtocode
 *  All Rights Reserved
 */

pub const NAME: &str = "ShareMusic";
pub const NAME_SHORT: &str = "Sharing";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod config_consts {
    pub const YAML_FILE_PATH: &str = "config.yaml";
    pub const JSON_FILE_PATH: &str = "config.json";

    pub const ENV_PREFIX: &str = "CONFIG.";
}

pub mod cluster_consts {
    use twilight_model::gateway::Intents;
    use twilight_model::gateway::payload::outgoing::update_presence::UpdatePresencePayload;
    use twilight_model::gateway::presence::{ActivityType, MinimalActivity, Status};

    pub const GATEWAY_INTENTS: Intents = Intents::GUILDS;

    pub fn presence() -> UpdatePresencePayload {
        UpdatePresencePayload {
            activities: vec![MinimalActivity {
                kind: ActivityType::Listening,
                name: "your requests!".to_string(),
                url: None
            }.into()],
            afk: false,
            since: None,
            status: Status::Online
        }
    }
}

pub mod state_consts {
    pub const QUEUE_LEN: usize = 5;
}

pub mod colour_consts {
    pub const MAX_IMAGE_SIZE: u32 = 4096;
}