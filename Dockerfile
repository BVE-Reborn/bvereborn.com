FROM alpine:latest

EXPOSE 8000

COPY static /app/static
COPY templates /app/templates
COPY Rocket.toml /app/Rocket.toml
COPY target/x86_64-unknown-linux-musl/release/bve-reborn-site /app/bve-reborn-site

WORKDIR "/app/"
ENTRYPOINT ["/app/bve-reborn-site"]
