# Start from the Rust build image to build our application.
FROM clux/muslrust:stable as builder

# Install musl-tools and g++ for building
RUN apt-get update && apt-get install -y musl-tools g++

# Set the compilation environment variables to use musl
ENV CC=musl-gcc
ENV CXX=g++

# Create a new empty shell project
WORKDIR /usr/src
RUN USER=root cargo new pricing
WORKDIR /usr/src/pricing

# Copy over your manifest
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source tree
COPY ./src ./src

# Build for release. 
# Previous steps cached dependencies, now it will only rebuild your code
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/pricing*
RUN cargo build --release

# Our final image
FROM gcr.io/distroless/static-debian11

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/pricing/target/x86_64-unknown-linux-musl/release/pricing /pricing

EXPOSE 8080

# Set the start command to run your binary
CMD ["/pricing"]