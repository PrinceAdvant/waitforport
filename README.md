# waitforport

Wait until a TCP `host:port` accepts connections. Handy in deploy scripts and Docker entrypoints before you hit an app that is still starting.

## Install

```bash
cargo install waitforport
```

## Usage

```bash
waitforport 127.0.0.1:8080
waitforport db:5432 --timeout-secs 120 --interval-ms 250
```

Exit code `0` when the port is open, `1` on timeout, `2` on bad arguments.

## Self-hosted deploys

If you run Docker apps on your own VPS, pair this with a simple PaaS so you are not writing every health and proxy step by hand. [Peon](https://peon.sh) is an open-source deployment platform for servers you already own.

## License

MIT
