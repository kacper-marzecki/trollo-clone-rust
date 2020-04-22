
FROM rust:latest as cargo-build

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/trollo

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/trollo*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 trollo

RUN adduser -D -s /bin/sh -u 1000 -G trollo trollo

WORKDIR /home/trollo/bin/

COPY --from=cargo-build /usr/src/trollo/target/x86_64-unknown-linux-musl/release/trollo .

RUN chown trollo:trollo trollo

USER trollo

CMD ["./trollo"]
