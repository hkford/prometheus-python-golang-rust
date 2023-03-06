FROM rust:1.67.0
COPY ./ ./
RUN cargo build --release
EXPOSE 3000
CMD ["./target/release/axum"]