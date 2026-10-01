use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
    AppState,
    handler::{
        create_note_handler, delete_note_handler, edit_note_handler, get_note_handler,
        health_checker_handler, note_list_handler,
    },
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route(
            "/api/notes",
            get(note_list_handler).post(create_note_handler),
        )
        .route(
            "/api/notes/{id}",
            get(get_note_handler)
                .patch(edit_note_handler)
                .delete(delete_note_handler),
        )
        .with_state(app_state)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{body::Body, http::{Request, StatusCode}};
    use sqlx::postgres::PgPoolOptions;
    use tower::ServiceExt;

    use super::create_router;
    use crate::AppState;

    #[tokio::test]
    async fn health_check_returns_ok() {
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://invalid:invalid@127.0.0.1/unused")
            .expect("valid database URL");
        let app = create_router(Arc::new(AppState::new(pool)));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/healthchecker")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn unknown_route_returns_not_found() {
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://invalid:invalid@127.0.0.1/unused")
            .expect("valid database URL");
        let app = create_router(Arc::new(AppState::new(pool)));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/does-not-exist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
