# Astropathic Relay

Highly secure and intensive data relay between two computers.

A simple and flexible Rust executable-based local transfer tool to send and receive text, images, videos, and files securely using native UIs. It connects over a live session so files can be streamed over without having to discretely "send" every time.

## Why?

- Multi-connection
- AES256 encryption
- Socket and packet options (streaming text, video, audio, images)
- Opt for fastest transfer path (SFP, LAN)
- Flexible UI modding

## Get Started

1. Generate your server key using `openssl`.

```sh
openssl req -x509 -newkey rsa:4096 -nodes -keyout server_key.pem -out server_cert.pem -days 365 -subj "/CN=localhost"
```

## License

Apache License 2.0
