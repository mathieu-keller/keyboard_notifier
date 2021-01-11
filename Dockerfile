# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

WORKDIR /usr/src/das_keyboard_github_service

COPY . .

RUN cargo build --release


# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 das_keyboard_github_service

RUN adduser -D -s /bin/sh -u 1000 -G das_keyboard_github_service das_keyboard_github_service

WORKDIR /home/das_keyboard_github_service/bin/

COPY --from=cargo-build /usr/src/das_keyboard_github_service/target/release/das_keyboard_github_service .

RUN chown das_keyboard_github_service:das_keyboard_github_service das_keyboard_github_service

USER das_keyboard_github_service

CMD ["./das_keyboard_github_service"]
