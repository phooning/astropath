# Architecture

Astropath is written under the assumption our local network is already compromised. Everything must be encrypted.

## Authentication

Astropath uses mTLS with pinned certificates to confirm machine identity.

## File Handling

Large video files use chunked transfer with identity verification per chunk.

## Memory Security

Astropath uses `zeroize` to clear memory after use.
