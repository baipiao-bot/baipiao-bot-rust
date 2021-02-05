FROM rust AS builder
COPY . /baipiao-bot-rust
WORKDIR /baipiao-bot-rust
RUN cargo build --release --example comment

FROM debian
MAINTAINER longfangsong@icloud.com
RUN apt update && apt install -y libssl-dev ca-certificates
COPY --from=builder /baipiao-bot-rust/target/release/examples/comment /
WORKDIR /
ENTRYPOINT ["/comment"]
