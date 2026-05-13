# proxy

I normally use `nginxproxy/nginx-proxy` as my proxy on my self-hosted projects, but it always needs to be on the same network with `VIRTUAL_HOST` set for each docker compose instance. Not a big issue, but not how I like my projects to behave. I'll learn a lot writing my own (yes, old skool without AI) and end up with something simple and performant.

## Development roadmap

- [ ] setup [hardened docker image](https://hub.docker.com/hardened-images/catalog)
- [x] add TOML config for domains (host: example.com, tls: self-signed|acme, upstream: 127.0.0.1:3000)
- [x] setup init process to parse config
- [ ] check SSL certs exist (rcgen, rustls-acme)
- [ ] start proxy with those values (planning to use pingora)

## AI disclaimer

For each public project I'll be upfront about what was done with AI.

For this one I don't want to use AI to write the code. I want to keep my skills sharp and have 100% understanding of every bit that went into it. That said, the following was done with AI:

- research about the project
- cleanup of the README and other text
- setup my zed config for linting/checking/security
- discussion about code solutions
