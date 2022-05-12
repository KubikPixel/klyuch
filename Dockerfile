FROM rust:latest
WORKDIR /usr/src/klyuch
COPY . .
RUN cargo install --path .
CMD ["klyuch"]
