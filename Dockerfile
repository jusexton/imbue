FROM rust as builder
WORKDIR imbue
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/imbue /usr/local/bin/imbue
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["imbue"]