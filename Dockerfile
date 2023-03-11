FROM rust:1.68

COPY ./ ./

RUN cargo build --release

EXPOSE 4040

CMD ["./target/release/msg-trinket"]