# Proposal: Fix Path Resolution and Error Mapping

## Problem
Users experience 403 Forbidden errors when they should see 404 Not Found, and relative path resolution in the browser fails (leading to "returning a level" behavior) when accessing directories without a trailing slash.

1. **Incorrect Error Mapping**: The server maps all `sanitize_path` errors (including `NotFound` from `canonicalize`) to `403 Forbidden`. This is misleading and makes debugging difficult.
2. **Missing Trailing Slash Redirect**: Directories accessed without a trailing slash (e.g., `/folder`) cause the browser to treat the directory as a file when resolving relative links (e.g., `sibling` becomes `/sibling` instead of `/folder/sibling`).
3. **Breadcrumb Links**: Current breadcrumb links point to paths without trailing slashes, triggering unnecessary redirects or contributing to the resolution issues.

## Solution
1. **Redirect Directories**: In the request handler, if a path resolves to a directory but does not end with a `/`, issue a `301 Moved Permanently` redirect to the slash-terminated version.
2. **Differentiate Errors**: Distinguish between `std::io::ErrorKind::NotFound` and other errors in the sanitization logic to return `404 Not Found` when appropriate.
3. **Update Breadcrumbs**: Modify the breadcrumb generation to include trailing slashes for directory links.

## Scope
- `src/handlers.rs`: Update `handle_request` to handle redirects and improve error mapping.
- `src/view.rs`: Update `render_breadcrumbs` to include trailing slashes.
- `src/fs_utils.rs`: Ensure `sanitize_path` provides enough context for error differentiation (already does via `anyhow` and `io::Error`).
