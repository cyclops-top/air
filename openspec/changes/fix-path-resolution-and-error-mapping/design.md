# Design: Path Resolution and Error Mapping Fix

## Overview
This change addresses path resolution issues and misleading error codes by enforcing directory trailing slashes and improving error handling in the request pipeline.

## Architectural Changes

### 1. Directory Trailing Slash Redirection
When a request matches a directory:
- Check if the URI path ends with `/`.
- If not, redirect (301) to `uri_path + "/"`.
- Preserve query strings if present.
- Use the raw `uri` from the request to ensure encoding is preserved during redirect.

### 2. Error Mapping Refinement
The `sanitize_path` function uses `canonicalize()`, which returns `std::io::Error`.
- In `handlers.rs`, inspect the error returned by `sanitize_path`.
- If the error is `io::ErrorKind::NotFound`, return `404 Not Found`.
- Otherwise, return `403 Forbidden` (as a catch-all for security violations or other access issues).

### 3. UI Breadcrumb Enhancement
Update `render_breadcrumbs` in `view.rs`:
- Ensure each generated link ends with a `/`.
- This avoids unnecessary redirects when navigating back through breadcrumbs.

## Trade-offs and Considerations
- **Redirect Overhead**: A small redirect overhead for the first access to a directory without a slash. This is standard practice in web servers (like Nginx/Apache).
- **Security**: Returning 404 instead of 403 for non-existent files is standard and doesn't leak more information than 403 already does in this context (since it's a file browser).
