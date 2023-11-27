FROM rust:latest

WORKDIR /usr/src/myapp
COPY . .

RUN RUST_LOG=debug cargo run --release 

CMD ["myapp"]
