FROM rust:1.79

RUN apt-get update && apt-get -y upgrade
RUN apt install --assume-yes -y protobuf-compiler

WORKDIR /usr/src/example_server
COPY . .
RUN cargo build --release
CMD ["/usr/src/example_server/target/release/example-server"]