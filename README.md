# Statsd Auth Proxy

A simple UDP proxy that authenticates packets using a token, and forwards them to a target address.
Made for statsd.

## Usage

```bash
./statsd_auth_proxy 
```

## With custom configuration
```bash
./statsd_auth_proxy --config config.json
```

## Debug mode (logs packets)
```bash
./statsd_auth_proxy --enable-debug
```

## Configuration

The configuration file is a JSON file with the following structure:

```json
{
  "tokens": [
    "123"
  ],
  "target": "127.0.0.1:1338",
  "port": 1337,
  "bind": "0.0.0.0"
}
```