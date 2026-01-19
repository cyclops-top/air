use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;
use serde::Serialize;
use crate::application::file_service::FileService;
use crate::domain::models::AppState;

pub struct ApiState {
    pub file_service: Arc<FileService>,
    pub app_state: Arc<AppState>,
}

/// 抽象 API 响应生成器
/// 未来可以在此根据 Accept Header 切换到 Protobuf 或其他格式
pub struct ApiResponse;

impl ApiResponse {
    pub fn ok<T: Serialize>(data: T, _headers: &HeaderMap) -> Response {
        // 目前仅支持 JSON，未来可扩展
        Json(data).into_response()
    }

    pub fn error(message: &str, status: StatusCode) -> Response {
        (status, Json(serde_json::json!({
            "error": message,
            "code": status.as_u16()
        }))).into_response()
    }
}

pub async fn handle_api_list(
    State(state): State<Arc<ApiState>>,
    headers: HeaderMap,
    Path(path): Path<String>,
) -> Response {
    // 1. 路径预处理 (类似于 handlers.rs)
    let decoded_path = match percent_encoding::percent_decode_str(&path).decode_utf8() {
        Ok(p) => p.to_string(),
        Err(_) => return ApiResponse::error("Invalid path encoding", StatusCode::BAD_REQUEST),
    };

    // 2. 构建绝对路径并校验安全性
    let abs_path = match crate::fs_utils::sanitize_path(&state.app_state.root_path, &decoded_path) {
        Ok(p) => p,
        Err(_) => return ApiResponse::error("Access denied", StatusCode::FORBIDDEN),
    };

    // 3. 调用应用层服务
    // 注意：API 不需要 /air 前缀，我们直接传递原始装饰后的路径或空
    match state.file_service.get_listing(&decoded_path, &abs_path).await {
        Ok(listing) => ApiResponse::ok(listing, &headers),
        Err(e) => ApiResponse::error(&e.to_string(), StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// 处理根目录 API 请求
pub async fn handle_api_list_root(
    state: State<Arc<ApiState>>,
    headers: HeaderMap,
) -> Response {
    handle_api_list(state, headers, Path("".to_string())).await
}
