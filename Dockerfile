FROM rust:1.28.0

WORKDIR /opt

COPY ./src ./src

COPY ./Cargo.toml .

RUN cargo build --release

EXPOSE 8008

CMD ["./target/release/scoping-tool-backend"]
