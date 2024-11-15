# Awala VPN Gateway Shield

The [Awala VPN](https://awala.app/en/vpn/) Gateway Shield,
or simply _shield_,
is a globally-distributed server powered by [Cloudflare Workers](https://workers.cloudflare.com/) to minimise latency with Awala VPN tunnels.
The shield is responsible for routing VPN traffic to a [gateway](https://github.com/AwalaApp/vpn-gateway),
protecting the gateway from [DDoS attacks](https://ddos.report),
enforcing usage quotas,
and resolving DNS records,
amongst other things.

To prevent eavesdropping from tunnel operators and other malicious actors,
this server exposes a single HTTP endpoint (`POST /`),
and all HTTP message payloads and WebSockets messages are to be end-to-end encrypted with the client.

## Operations

### Web browsing simulation

The server supports two operations to help the client simulate that it's browsing the tunnel:

- `web_browsing_simulation.webpage`: This returns a HTTP response that simulates a webpage. It actually contains the instructions to make requests to `web_browsing_simulation.asset`.
- `web_browsing_simulation.asset`: This returns a HTTP response that simulates a web asset (e.g. an image, JavaScript file).

## Licence

This project is fair source.
See [`LICENSE`](LICENSE).
