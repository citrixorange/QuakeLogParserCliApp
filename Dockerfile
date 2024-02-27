FROM rust:1.76
LABEL image.name="quake-log-parser-cli:v1"
WORKDIR /app
RUN adduser user && chown -R user /app
USER user
COPY . .
RUN rustc --version
RUN cargo --version
RUN cargo build --release
WORKDIR /app/target/release