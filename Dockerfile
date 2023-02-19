

FROM rust

WORKDIR /usr/src/rust-react-chat
COPY . .

RUN apt-get update
RUN apt-get install libpq-dev -y
RUN cargo install --path .
# EXPOSE 8080
CMD ["rust-react-chat"]
