FROM alpine:latest AS build

# Install software
RUN apk update && apk upgrade && apk add --no-cache curl musl-dev gcc

# Add to non-root user
RUN adduser -D -u 1000 rasopus

# Copy source code from host
RUN mkdir /app
WORKDIR /app
COPY ./src /app/src
COPY ./migrations /app/migrations
COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock
COPY ./rust-toolchain.toml /app/rust-toolchain.toml
RUN chown -R rasopus:rasopus /app
RUN chmod -R 755 /app

# Switch to non-root user
USER rasopus

# Install Rust toolchain for user
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/rasopus/.cargo/bin:${PATH}"

# Build the project
RUN cargo build --release

################################################################################
FROM alpine:latest AS runtime

# Install software
RUN apk update && apk upgrade

# Uninstall non-necessary software
RUN rm -rf /var/cache/apk/* && rm -rf /tmp/* && rm /sbin/apk

# Create a non-root user
RUN adduser -D -u 1000 rasopus
USER rasopus

# Copy necessary files from the build container
WORKDIR /app
COPY --from=build /app/target/release/rasopus .

# Default command
CMD ["./rasopus"]