# Tasks: Path Resolution and Error Mapping Fix

- [x] Improve error mapping in `src/handlers.rs`
    - [x] Update `handle_request` to downcast `sanitize_path` errors.
    - [x] Return `StatusCode::NOT_FOUND` if `io::ErrorKind::NotFound`.
- [x] Implement directory trailing slash redirect in `src/handlers.rs`
    - [x] Detect if directory request lacks trailing slash.
    - [x] Issue `301 Moved Permanently` redirect.
    - [x] Preserve query parameters.
- [x] Update breadcrumb generation in `src/view.rs`
    - [x] Modify `render_breadcrumbs` to append `/` to all directory links.
    - [x] Update unit tests for breadcrumbs.
- [x] Verification
    - [x] Manually verify accessing a folder without a slash redirects.
    - [x] Verify breadcrumb links work correctly.
    - [x] Verify non-existent files return 404.
    - [x] Verify traversal attempts still return 403.