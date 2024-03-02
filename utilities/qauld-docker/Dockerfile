FROM rust:1.68.2-bullseye AS builder

RUN apt-get update
RUN apt-get install -y git protobuf-compiler

RUN git clone https://github.com/qaul/qaul.net.git /qaul

WORKDIR /qaul/rust

RUN cargo build --release


FROM alpine:3.17 AS runner

RUN apk add gcompat libstdc++

ENV GID 1000
ENV UID 1000
ENV PORT 9229
ENV NAME "Self-hosted Qaul"

COPY --from=builder --chown=${UID}:${GID} /qaul/rust/target/release/qauld /bin

EXPOSE ${PORT}
USER ${UID}:${GID}

WORKDIR /srv/qaul

ENV RUST_BACKTRACE 1
ENTRYPOINT [ "qauld", "--name=", "$NAME", "--port", "$PORT" ]
