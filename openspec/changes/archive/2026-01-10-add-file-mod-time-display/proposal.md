# Proposal: Display File Modification Time and Ensure Cache Invalidation

## Why
Users need to see when files were last modified directly in the Web UI to manage their files effectively. Additionally, ensuring that any change in the modification time triggers a re-calculation of the SHA-256 digest is critical for data integrity and correct cache behavior (ETags).

## What Changes
- **Web UI Enhancement**: Add a "Last Modified" column to the directory listing page.
- **Cache Invalidation Verification**: Ensure and document that the server invalidates the SHA-256 digest cache whenever a file's modification time or size changes.

## Impact
- **Web UI**: Users will see a new timestamp for each file/directory.
- **Server**: No logic change needed for invalidation (already implemented), but the behavior is now formally specified and tested.
