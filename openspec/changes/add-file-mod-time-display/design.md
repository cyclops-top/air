# Design: File Modification Time Display

## Web UI Changes
- **HTML Template**: Modify `src/view.rs` to include a new `<span>` or column for the modification time in each file entry.
- **Styling**: Add CSS to ensure the modification time is displayed clearly (e.g., secondary color, aligned next to or below the file name/size).
- **Time Formatting**: The modification time is already provided as an RFC 3339 string by the backend. We will display this string, possibly simplified for readability.

## Cache Invalidation Logic (Verification)
The `src/handlers.rs` already contains the following logic:
```rust
if let Some(entry) = state.digest_cache.get(&abs_path) {
    if entry.mtime == mtime && entry.size == size {
        entry.hash.clone()
    } else {
        // ... recalculate ...
    }
}
```
This design confirms this behavior as a requirement: Any mismatch in `mtime` (Modification Time) or `size` MUST result in the deletion/invalidation of the cached SHA-256 digest and a forced re-calculation.

## Data Model
The `FileEntry` struct already includes `mod_time: String`, so no changes to the data transfer object (DTO) are required.
