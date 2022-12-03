FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /crc-rs
WORKDIR /crc-rs/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /crc-rs/fuzz/target/x86_64-unknown-linux-gnu/release/crc-rs-fuzz /