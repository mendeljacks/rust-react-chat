

FROM rust

WORKDIR /usr/src/rust-react-chat
COPY . .

RUN apt-get update
RUN apt-get install sqlite3 -y
RUN cargo install --path .
# EXPOSE 8080
CMD ["rust-react-chat"]
