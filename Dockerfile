# Use the official Rust nightly image
FROM rust:nightly

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Pre-build the dependencies to cache them
RUN echo "fn main() {}" > dummy.rs
RUN mkdir src && mv dummy.rs src/
RUN cargo build --release
RUN rm -rf src/dummy.rs

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Expose the port Rocket will use
EXPOSE 8000

# Command to run the application
CMD ["./target/release/rocket_server"]
