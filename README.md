# Awala VPN Gateway Shield

The [Awala VPN](https://awala.app/en/vpn/tech-overview/) Gateway Shield,
or simply _shield_,
is a middleware between an Awala VPN [client](https://awala.app/en/vpn/tech-overview/#client) and its [gateway](https://awala.app/en/vpn/tech-overview/#gateway),
via a [tunnel](https://awala.app/en/vpn/tech-overview/#tunnel).
Its responsibilities include:

- Brokering connections between clients and gateways.
- Mitigating [DDoS attacks](https://ddos.report).
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

## DDoS Attacks Mitigation

1. Internet layer mitigation: Host the app on a provider that provides [volumetric attack](https://ddos.report/overview/#volumetric-attacks) protection. Even better if they support BGP blackholing, to drop traffic from outside the shield's _catchment area_.
2. Protocol layer mitigation: Run the app behind a mainstream reverse proxy (e.g., Caddy) for [protocol-level protection](https://ddos.report/overview/#protocol-attacks), which should be configured to:
   - Drop connections from IP addresses outside the catchment area.
   - Drop connections from known malicious IP addresses (e.g. using FireHOL IP Lists).
   - Only allow connections from data centres. Residential IP addresses should be blocked.
   - Rate-limit TCP connections per IP address, in a distributed manner.
3. Application layer mitigation: Integrate [Despacito](https://despacito.bot/) in the shield app for [application-layer protection](https://ddos.report/overview/#application-attacks).

See also the [Awala VPN threat model](https://awala.app/en/vpn/tech-overview/#threat-model).

## Licence

This project is fair source.
See [`LICENSE`](LICENSE).
