# Awala VPN Gateway Shield

The [Awala VPN](https://awala.app/en/vpn/tech-overview/) Gateway Shield,
or simply _shield_,
is a middleware between an Awala VPN [client](https://awala.app/en/vpn/tech-overview/#client) and its [gateway](https://awala.app/en/vpn/tech-overview/#gateway).
The shield is powered by [Cloudflare Workers](https://workers.cloudflare.com/) to run at the edge.
Its responsibilities include:

- Brokering connections between clients and gateways.
- Using [Despacito](https://despacito.bot/) to protect gateways from [application DDoS attacks](https://ddos.report/overview/#application-attacks).
- Enforcing usage quotas.
- Resolving DNS records.
- Simulating web browsing to obfuscate VPN traffic.

## Client-Shield Protocol

The client and shield communicate over an end-to-end encrypted channel using Diffie-Hellman with X25519 keys.
Key pairs are rotated every 60 minutes or when the client starts.
The shield rejects requests with key pairs older than 60 minutes.

To prevent replay attacks from malicious tunnels, each HTTP request includes a monotonically increasing sequence number,
which is verified before processing and included in responses.
The sequence number is tied to the client's key pair and resets to zero upon key rotation.

HTTP requests and responses are padded to random sizes to mitigate traffic analysis.

### Web Browsing Simulation

The shield supports two operations to help clients simulate web browsing:

- `web_browsing_simulation.webpage`: Returns a HTTP response simulating a webpage, containing instructions to request assets.
- `web_browsing_simulation.asset`: Returns a HTTP response simulating a web asset (e.g., image, JavaScript file).

### WebSocket Connection

After simulating webpage retrieval,
clients establish a WebSocket connection to relay IP packets.
Both parties maintain encryption keys for the connection duration, even if key rotation occurs.

To prevent replay attacks,
each encrypted message includes the HTTP request sequence number and a monotonically increasing sequence number.
The shield maintains a separate sequence for messages it sends.

The connection also carries control messages (e.g., subscription data usage updates).

To mitigate traffic analysis, the shield:

- Exchanges bidirectional noise frames of random sizes at random intervals.
- Pads each packet to a random size and/or batches packets.
- Delays the first message from the gateway.

## Shield-Gateway Protocol

When a client starts a packet relay connection, the shield:

1. Selects a gateway by reusing an existing session or allocating a new one.
2. Relays E2E encrypted IP packets bidirectionally between client and gateway until either party closes the connection.

## Licence

This project is fair source.
See [`LICENSE`](LICENSE).
