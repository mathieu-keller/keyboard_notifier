FROM alpine:latest

WORKDIR /home/notifier/bin/
COPY target/release/ .
CMD ["./das_keyboard_github_service"]
