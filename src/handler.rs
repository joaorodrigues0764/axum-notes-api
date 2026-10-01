use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use tracing::error;
use uuid::Uuid;

use crate::{
    AppState,
    error::{ApiError, ApiResult},
    model::NoteModel,
    schema::{CreateNoteSchema, FilterOptions, UpdateNoteSchema},
};

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 100;
const MAX_TITLE_LENGTH: usize = 200;
const MAX_CONTENT_LENGTH: usize = 10_000;
const MAX_CATEGORY_LENGTH: usize = 100;

pub async fn health_checker_handler() -> impl IntoResponse {
    Json(json!({
        "status": "success",
        "message": "API is healthy"
    }))
}

pub async fn note_list_handler(
    Query(opts): Query<FilterOptions>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<impl IntoResponse> {
    let page = opts.page.unwrap_or(1);
    let limit = opts.limit.unwrap_or(DEFAULT_LIMIT);

    if page == 0 {
        return Err(ApiError::bad_request("page must be greater than 0"));
    }

    if limit == 0 || limit > MAX_LIMIT {
        return Err(ApiError::bad_request(format!(
            "limit must be between 1 and {MAX_LIMIT}"
        )));
    }

    let limit = i64::from(limit);
    let offset = i64::from(page - 1).saturating_mul(limit);

    let notes = sqlx::query_as!(
        NoteModel,
        "SELECT * FROM notes ORDER BY created_at DESC, id DESC LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(&state.db)
    .await
    .map_err(|err| {
        error!(error = ?err, "Failed to fetch notes");
        ApiError::Internal
    })?;

    Ok(Json(json!({
        "status": "success",
        "results": notes.len(),
        "page": page,
        "limit": limit,
        "notes": notes
    })))
}

pub async fn create_note_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateNoteSchema>,
) -> ApiResult<impl IntoResponse> {
    let title = body.title.trim();
    let content = body.content.trim();

    validate_text("title", title, 1, MAX_TITLE_LENGTH)?;
    validate_text("content", content, 1, MAX_CONTENT_LENGTH)?;

    let category = normalize_category(body.category)?;
    let published = body.published.unwrap_or(false);

    let note = sqlx::query_as!(
        NoteModel,
        "INSERT INTO notes (title, content, category, published) VALUES ($1, $2, $3, $4) RETURNING *",
        title,
        content,
        category,
        published
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| {
        error!(error = ?err, "Failed to create note");
        ApiError::Internal
    })?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "status": "success",
            "data": { "note": note }
        })),
    ))
}

pub async fn get_note_handler(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<impl IntoResponse> {
    let result = sqlx::query_as!(NoteModel, "SELECT * FROM notes WHERE id = $1", id)
        .fetch_one(&state.db)
        .await;

    match result {
        Ok(note) => Ok(Json(json!({
            "status": "success",
            "data": { "note": note }
        }))),
        Err(sqlx::Error::RowNotFound) => {
            Err(ApiError::not_found(format!("Note with ID {id} not found")))
        }
        Err(err) => {
            error!(error = ?err, %id, "Failed to fetch note");
            Err(ApiError::Internal)
        }
    }
}

pub async fn edit_note_handler(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<UpdateNoteSchema>,
) -> ApiResult<impl IntoResponse> {
    if body.title.is_none()
        && body.content.is_none()
        && body.category.is_none()
        && body.published.is_none()
    {
        return Err(ApiError::bad_request(
            "At least one field must be provided",
        ));
    }

    let title = match body.title.as_deref() {
        Some(value) => {
            let value = value.trim();
            validate_text("title", value, 1, MAX_TITLE_LENGTH)?;
            Some(value)
        }
        None => None,
    };

    let content = match body.content.as_deref() {
        Some(value) => {
            let value = value.trim();
            validate_text("content", value, 1, MAX_CONTENT_LENGTH)?;
            Some(value)
        }
        None => None,
    };

    let category = match body.category.as_deref() {
        Some(value) => {
            let value = value.trim();
            if value.chars().count() > MAX_CATEGORY_LENGTH {
                return Err(ApiError::bad_request(format!(
                    "category must be at most {MAX_CATEGORY_LENGTH} characters long"
                )));
            }
            Some(value)
        }
        None => None,
    };

    let note = sqlx::query_as!(
        NoteModel,
        "UPDATE notes
         SET title = COALESCE($1, title),
             content = COALESCE($2, content),
             category = COALESCE($3, category),
             published = COALESCE($4, published),
             updated_at = CURRENT_TIMESTAMP
         WHERE id = $5
         RETURNING *",
        title,
        content,
        category,
        body.published,
        id
    )
    .fetch_one(&state.db)
    .await;

    match note {
        Ok(note) => Ok(Json(json!({
            "status": "success",
            "data": { "note": note }
        }))),
        Err(sqlx::Error::RowNotFound) => {
            Err(ApiError::not_found(format!("Note with ID {id} not found")))
        }
        Err(err) => {
            error!(error = ?err, %id, "Failed to update note");
            Err(ApiError::Internal)
        }
    }
}

pub async fn delete_note_handler(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<impl IntoResponse> {
    let result = sqlx::query!("DELETE FROM notes WHERE id = $1", id)
        .execute(&state.db)
        .await
        .map_err(|err| {
            error!(error = ?err, %id, "Failed to delete note");
            ApiError::Internal
        })?;

    if result.rows_affected() == 0 {
        return Err(ApiError::not_found(format!("Note with ID {id} not found")));
    }

    Ok(StatusCode::NO_CONTENT)
}

fn validate_text(
    field: &str,
    value: &str,
    min_length: usize,
    max_length: usize,
) -> ApiResult<()> {
    let length = value.chars().count();

    if length < min_length {
        return Err(ApiError::bad_request(format!(
            "{field} cannot be empty"
        )));
    }

    if length > max_length {
        return Err(ApiError::bad_request(format!(
            "{field} must be at most {max_length} characters long"
        )));
    }

    Ok(())
}

fn normalize_category(category: Option<String>) -> ApiResult<Option<String>> {
    match category {
        Some(value) => {
            let value = value.trim();
            if value.is_empty() {
                return Ok(None);
            }

            if value.chars().count() > MAX_CATEGORY_LENGTH {
                return Err(ApiError::bad_request(format!(
                    "category must be at most {MAX_CATEGORY_LENGTH} characters long"
                )));
            }

            Ok(Some(value.to_owned()))
        }
        None => Ok(None),
    }
}

