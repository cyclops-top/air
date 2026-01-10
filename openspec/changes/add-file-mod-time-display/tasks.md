# Tasks: File Modification Time Display

- [x] Update Web UI in `src/view.rs`
    - [x] Add modification time display to `render_html`.
    - [x] Add styling for the modification time.
- [x] Formalize Cache Invalidation
    - [x] Add a unit test in `src/handlers.rs` or `src/fs_utils.rs` (if applicable) specifically verifying that changing modification time invalidates the cache (if tests can easily mock state).
- [x] Verification
    - [x] Verify modification time is visible in the browser.
    - [x] Verify that modifying a file's content (which updates its mtime) results in a new ETag/Digest.