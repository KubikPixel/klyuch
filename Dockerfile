FROM rust
WORKDIR /usr/src/klyuch
COPY . .
RUN cargo install --path .
CMD ["klyuch"]
