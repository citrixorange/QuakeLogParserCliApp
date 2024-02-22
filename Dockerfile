FROM alpine:latest
USER root
LABEL image.name="quake-log-parser-cli:v1"
WORKDIR /app
COPY . .
RUN ./setup.sh
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustc --version
RUN cargo --version
RUN cargo build --release
WORKDIR /app/target/release