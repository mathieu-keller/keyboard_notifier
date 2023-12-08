FROM alpine:3.19

USER 1000:1000

WORKDIR /home/notifier/bin/
COPY --chown=1000:1000 target/x86_64-unknown-linux-musl/release/das_keyboard_github_service .
CMD ["./das_keyboard_github_service"]
