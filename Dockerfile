FROM alpine:3.15.4

WORKDIR /home/notifier/bin/
COPY target/x86_64-unknown-linux-musl/release/das_keyboard_github_service .
CMD ["./das_keyboard_github_service"]
