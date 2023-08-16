FROM rust:latest
WORKDIR /usr/src/hello_rs_docker
COPY . .
RUN cargo install --path .
EXPOSE 8080/tcp
CMD ["hello_rs_docker"]