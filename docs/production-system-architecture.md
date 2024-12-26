# Awala VPN Shield's Production System Architecture

- Reverse Proxy: Caddy
- Third-party backing services:
  - Amazon Cloudwatch for observability (via OpenTelemetry).
  - Encryption key store: Custom service that manages the encryption keys with the clients. Hosted on AWS Lambda, and using Amazon KMS and MongoDB Atlas. This service will also be responsible for periodically rotating unbound keys, and pushing the public keys to the Orchestrator.
