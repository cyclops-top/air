# Proposal: Implement Memory-Mapped File Sharing Cache

## Problem Statement
The current implementation uses `tower_http::services::ServeFile` for file serving. While robust, it performs traditional file I/O for every request. For high-concurrency scenarios, especially with large files and many range requests (e.g., video streaming or chunked downloads), memory mapping (`mmap`) can significantly improve performance by leveraging the OS page cache and providing zero-copy data transfer to the network stack.

## Proposed Solution
Introduce a centralized `MmapCache` that manages memory mappings of files being served.
- **Shared Mappings**: Multiple concurrent requests for the same file will share a single memory mapping.
- **Reference Counting**: Mappings are automatically released when the last request using them completes.
- **Zero-Copy**: Serve file content directly from the memory map to minimize CPU usage and memory copying.
- **Thread Safety**: Use `DashMap` and fine-grained locking to handle concurrent access and prevent duplicate mapping creation.

## User Impact
- **Performance**: Improved throughput and reduced latency for file downloads.
- **Efficiency**: Lower CPU and memory overhead during high-concurrency file serving.
- **Reliability**: Faster response times for Range requests, beneficial for media playback.
