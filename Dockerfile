FROM alpine:latest

WORKDIR /home/notifier/bin/
COPY target/x86_64-unknown-linux-musl/release/ .
CMD ["./das_keyboard_github_service"]
