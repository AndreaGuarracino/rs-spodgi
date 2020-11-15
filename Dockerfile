FROM rust:1.47 as rust

WORKDIR /rs-spodgi

COPY . .

RUN cargo build --release

FROM debian:stable-slim

COPY --from=rust /rs-spodgi/target/release/rs-spodgi /usr/bin

ENTRYPOINT [ "rs-spodgi" ]

