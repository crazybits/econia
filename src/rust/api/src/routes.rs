use axum::{extract::State, routing::get, Json, Router};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use types::error::TypeError;

use crate::{error::ApiError, ws::ws_handler, AppState};

mod account_routes;
mod market_routes;

pub fn router(state: AppState) -> Router {
    let cors_layer = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let middleware_stack = ServiceBuilder::new()
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer);

    Router::new()
        .route("/", get(index))
        .route("/markets", get(markets))
        .route(
            "/account/:account_address/order-history",
            get(account_routes::order_history_by_account),
        )
        .route(
            "/account/:account_address/open-orders",
            get(account_routes::open_orders_by_account),
        )
        .route(
            "/market/:market_id/history",
            get(market_routes::get_market_history),
        )
        .route("/market/:market_id/fills", get(market_routes::get_fills))
        .route("/ws", get(ws_handler))
        .with_state(state)
        .layer(middleware_stack)
}

async fn index() -> String {
    String::from("Econia backend API")
}

async fn markets(State(state): State<AppState>) -> Result<Json<Vec<types::Market>>, ApiError> {
    let query_markets = sqlx::query_as!(
        types::query::QueryMarket,
        r#"
        select
            market_id,
            base.name as "base_name?",
            base.symbol as "base_symbol?",
            base.decimals as "base_decimals?",
            base_account_address,
            base_module_name,
            base_struct_name,
            base_name_generic,
            quote.name as quote_name,
            quote.symbol as quote_symbol,
            quote.decimals as quote_decimals,
            quote_account_address,
            quote_module_name,
            quote_struct_name,
            lot_size,
            tick_size,
            min_size,
            underwriter_id,
            created_at
        from markets
            left join coins base on markets.base_account_address = base.account_address
                                and markets.base_module_name = base.module_name
                                and markets.base_struct_name = base.struct_name
            join coins quote on markets.quote_account_address = quote.account_address
                                and markets.quote_module_name = quote.module_name
                                and markets.quote_struct_name = quote.struct_name;
        "#
    )
    .fetch_all(&state.pool)
    .await?;

    let markets = query_markets
        .into_iter()
        .map(|v| v.try_into())
        .collect::<Result<Vec<types::Market>, TypeError>>()?;

    Ok(Json(markets))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use sqlx::PgPool;
    use tokio::sync::broadcast;
    use tower::ServiceExt;

    use super::*;
    use crate::load_config;

    #[tokio::test]
    async fn test_index() {
        let app = Router::new().route("/", get(index));

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"Econia backend API");
    }

    #[tokio::test]
    async fn test_markets() {
        let config = load_config();

        let pool = PgPool::connect(&config.database_url)
            .await
            .expect("Could not connect to DATABASE_URL");

        let (tx, _) = broadcast::channel(16);

        let state = AppState { pool, sender: tx };

        let app = Router::new()
            .route("/markets", get(markets))
            .with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/markets")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let s = String::from_utf8(body.to_vec()).unwrap();
        let result: Result<Vec<types::Market>, serde_json::Error> =
            serde_json::from_str(s.as_str());

        assert!(result.is_ok());
    }
}