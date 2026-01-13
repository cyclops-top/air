# Proposal: Add Service Discovery

## Goal
Implement a UDP-based service discovery mechanism to allow Air nodes to find each other on the local network. This includes a background broadcaster for servers and a `discover` command for clients.

## Motivation
Manually typing IP addresses and ports is cumbersome for local file sharing. Service discovery enables a "zero-config" experience where users can see available shares instantly.

## Scope
- Implement a UDP broadcaster in the Air server.
- Implement a `discover` CLI command to list active Air nodes.
- Support macOS/Linux/Windows with proper socket options (`SO_REUSEPORT`/`SO_REUSEADDR`).
- Use JSON-based payloads for extensibility.

## Out of Scope
- Service discovery over WAN or non-local subnets.
- Authentication or encryption for discovery packets (public info only).
