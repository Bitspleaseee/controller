# Build image
FROM rustlang/rust:nightly as build

WORKDIR /usr/src/controller

# Copy manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Copy source tree
COPY ./src ./src

# Build for release
RUN cargo build --release

# Final image
FROM debian:stable-slim

# Install mariadb client
RUN apt-get update
RUN apt-get -y install libmariadbclient-dev

# Copy the binaries
WORKDIR /usr/src/
COPY --from=build /usr/src/controller/target/release/controller .
COPY --from=build /usr/src/controller/target/release/inspector .

# Set the startup command to run the binary
CMD ["./controller", "-m", "-v", "-v"]