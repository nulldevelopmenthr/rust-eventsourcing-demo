FROM rust:1.33

RUN mkdir /target /dummy_app

RUN rustup component add rustfmt

COPY . /dummy_app

WORKDIR /dummy_app
RUN cargo fetch
RUN cargo build
RUN cargo check

CMD ["sleep", "1d"]