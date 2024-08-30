FROM rust

RUN cargo install rustlings
RUN rustup component add clippy

WORKDIR /root
COPY entrypoint.sh .

# docker build -t rustlings .
# docker run --rm -it -v "${PWD}/mount:/root/mount" --name rustlings rustlings
# docker exec -it rust-sdk bash
CMD ["./entrypoint.sh"]