# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

WORKDIR /usr/src/das_keyboard_github_service

COPY . .

RUN cargo build --release
EXPOSE 8080
CMD ["/usr/src/das_keyboard_github_service/target/release/das_keyboard_github_service"]

