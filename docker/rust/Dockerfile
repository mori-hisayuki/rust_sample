FROM rust:1.43.0

ARG UID
ARG GID
ARG UNAME

ENV UID ${UID}
ENV GID ${GID}
ENV UNAME ${UNAME}

RUN groupadd -g ${GID} ${UNAME}
RUN useradd -u ${UID} -g ${UNAME} -m ${UNAME}

COPY ./rust/Cargo.toml /home/${UNAME}

RUN mkdir src
RUN echo "fn main() {}" >> src/main.rs
RUN cargo install \
    cargo-edit \
    cargo-watch \
    catflap \

EXPOSE 8080

USER ${UNAME}
CMD ["catflap", "--", "cargo", "watch", "-x", "run"]