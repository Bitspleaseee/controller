FROM rustlang/rust:nightly

WORKDIR /usr/src/controller
COPY . .

RUN cargo install --path .

CMD ["controller"]
