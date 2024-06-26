FROM rust:1.74.1

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

WORKDIR /app
COPY . .

RUN rustup default nightly
RUN cargo build --release

CMD ["/app/target/release/sys_monitor"]