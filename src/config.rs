#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    // pub jwt_expires_in: String,
    // pub jwt_max_age: i32,

    pub s3_key: String,
    pub s3_secret: String,
    pub s3_bucket: String,
    pub s3_region: String,
    pub s3_endpoint: String,

    pub livekit_server_url: String,
    pub livekit_turn_url: String,
    pub livekit_api_key: String,
    pub livekit_secret_key: String,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        // let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        // let jwt_max_age = std::env::var("JWT_MAX_AGE").expect("JWT_MAX_AGE must be set");

        let s3_key = std::env::var("S3_KEY").expect("S3_KEY must be set");
        let s3_secret = std::env::var("S3_SECRET").expect("S3_SECRET must be set");
        let s3_bucket = std::env::var("S3_BUCKET").expect("S3_BUCKET must be set");
        let s3_region = std::env::var("S3_REGION").expect("S3_REGION must be set");
        let s3_endpoint = std::env::var("S3_ENDPOINT").expect("S3_ENDPOINT must be set");

        let livekit_server_url = std::env::var("LIVEKIT_SERVER_URL").expect("LIVEKIT_SERVER_URL must be set");
        let livekit_turn_url = std::env::var("LIVEKIT_TURN_URL").expect("LIVEKIT_TURN_URL must be set");
        let livekit_api_key = std::env::var("LIVEKIT_API_KEY").expect("LIVEKIT_API_KEY must be set");
        let livekit_secret_key = std::env::var("LIVEKIT_SECRET_KEY").expect("LIVEKIT_SECRET_KEY must be set");

        return Config {
            database_url,
            jwt_secret,
            // jwt_expires_in,
            // jwt_max_age: jwt_max_age.parse::<i32>().unwrap(),

            s3_key,
            s3_secret,
            s3_bucket,
            s3_region,
            s3_endpoint,

            livekit_server_url,
            livekit_turn_url,
            livekit_api_key,
            livekit_secret_key
        };
    }
}
