FROM alpine:latest

WORKDIR /home/notifier/bin/
COPY target/release/das_keyboard_github_service .
CMD ["./das_keyboard_github_service"]
