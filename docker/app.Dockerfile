FROM rust:latest as base

WORKDIR /build
COPY Cargo.* .
RUN mkdir src && echo "fn main() {}" >> src/main.rs

FROM base as cache-debug
RUN cargo b

FROM base as cache-release-static
ENV STATIC_BUILD_TARGET x86_64-unknown-linux-musl
RUN rustup target add ${STATIC_BUILD_TARGET}
RUN cargo b --release --target=${STATIC_BUILD_TARGET}

FROM cache-debug as debug

COPY src src

RUN cargo b

FROM cache-release-static as release

COPY src src

RUN cargo b --release --target=${STATIC_BUILD_TARGET}

FROM rust:latest as deploy

COPY --from=release /build/target/x86_64-unknown-linux-musl/release/todo-pg /todo-pg
EXPOSE 8080

# ENTRYPOINT [ "/app" ]
