FROM rustlang/rust:nightly

WORKDIR /usr/src/controller
COPY . .

RUN cargo install --path . --bin controller

CMD ["controller", "-m", "-v"]
