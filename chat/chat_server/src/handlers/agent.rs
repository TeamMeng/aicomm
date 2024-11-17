use crate::{AppError, AppState, CreateAgent, UpdateAgent};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

/// List of all agents in chat
pub(crate) async fn list_agent_handler(
    Path(chat_id): Path<u64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let agents = state.list_agents(chat_id).await?;
    Ok((StatusCode::OK, Json(agents)))
}

/// Create a new agent in the chat
pub(crate) async fn create_agent_handler(
    Path(chat_id): Path<u64>,
    State(state): State<AppState>,
    Json(input): Json<CreateAgent>,
) -> Result<impl IntoResponse, AppError> {
    let agent = state.create_agent(input, chat_id).await?;
    Ok((StatusCode::CREATED, Json(agent)))
}

/// Update an agent by id
pub(crate) async fn update_agent_handler(
    Path(chat_id): Path<u64>,
    State(state): State<AppState>,
    Json(input): Json<UpdateAgent>,
) -> Result<impl IntoResponse, AppError> {
    let agent = state.update_agent(input, chat_id).await?;
    Ok((StatusCode::OK, Json(agent)))
}
