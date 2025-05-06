use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

/// Starting capacity for the HashMap that we use to store `Player` data.
static MAP_CAPACITY: usize = 500;

/// THRESHOLD that determines when we should remove `Player` after logout.
/// Note: max selectable update interval is 60 seconds so pad accordingly.
static LOGOUT_THRESHOLD_SECS: i64 = 80;

#[derive(Debug)]
struct Data {
    players: HashMap<String, Player>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PlayerRequest {
    /// Username of the player that sent the request payload.
    name: String,
    /// Player's ID.
    id: i32,
    /// Current X-Axis position.
    x: i32,
    /// Current Y-Axis position.
    y: i32,
    /// Current Z-Axis position.
    z: i32,
    /// Current world the player is logged into.
    w: i32,
    /// Region ID adjusted for local instanced regions.
    r: i32,
    /// Current HP.
    #[serde(rename(deserialize = "hm", serialize = "hm"))]
    current_hitpoints: i32,
    /// Hitpoints level.
    #[serde(rename(deserialize = "hM", serialize = "hM"))]
    hitpoints_level: i32,
    /// Current prayer.
    #[serde(rename(deserialize = "pm", serialize = "pm"))]
    current_prayer: i32,
    /// Prayer level.
    #[serde(rename(deserialize = "pM", serialize = "pM"))]
    prayer_level: i32,
    /// Added friends to check for.
    friends: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Player {
    /// Username of the player that sent the request payload.
    name: String,
    /// Player's ID.
    id: i32,
    /// Current X-Axis position.
    x: i32,
    /// Current Y-Axis position.
    y: i32,
    /// Current Z-Axis position.
    z: i32,
    /// Current world the player is logged into.
    w: i32,
    /// Region ID adjusted for local instanced regions.
    r: i32,
    /// Current HP.
    #[serde(rename(deserialize = "hm", serialize = "hm"))]
    current_hitpoints: i32,
    /// Hitpoints level.
    #[serde(rename(deserialize = "hM", serialize = "hM"))]
    hitpoints_level: i32,
    /// Current prayer.
    #[serde(rename(deserialize = "pm", serialize = "pm"))]
    current_prayer: i32,
    /// Prayer level.
    #[serde(rename(deserialize = "pM", serialize = "pM"))]
    prayer_level: i32,
    #[serde(skip)]
    last_updated: Timestamp,
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("This shall never fail us.");
    let state = Arc::new(Mutex::new(Data {
        players: HashMap::with_capacity(MAP_CAPACITY),
    }));
    let app = Router::new()
        .route("/", get(root))
        .route("/friends", post(friends))
        .with_state(state);
    Ok(app.into())
}

async fn friends(
    State(state): State<Arc<Mutex<Data>>>,
    Json(requester): Json<PlayerRequest>,
) -> Result<Json<Vec<Player>>, StatusCode> {
    let mut state = state.lock().map_err(|e| {
        tracing::error!("Error acquiring MutexGuard: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    info!(
        "{} with an id of {} has asked to relay information.",
        requester.name, requester.id
    );

    state.players.insert(
        requester.name.clone(),
        Player {
            name: requester.name,
            id: requester.id,
            x: requester.x,
            y: requester.y,
            z: requester.z,
            w: requester.w,
            r: requester.r,
            current_hitpoints: requester.current_hitpoints,
            hitpoints_level: requester.hitpoints_level,
            current_prayer: requester.current_prayer,
            prayer_level: requester.prayer_level,
            last_updated: Timestamp::now(),
        },
    );

    let time_now = Timestamp::now();
    state
        .players
        .retain(|_, value| (time_now - value.last_updated).get_seconds() < LOGOUT_THRESHOLD_SECS);

    let list = state
        .players
        .values()
        .filter(|&player| requester.friends.contains(&player.name))
        .cloned()
        .collect::<Vec<Player>>();
    Ok(Json(list))
}

async fn root() -> &'static str {
    "Nothing interesting happens."
}
