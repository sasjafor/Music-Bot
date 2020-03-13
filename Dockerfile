FROM rust:1.40

# Install rust toolchain
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Setup apt, install package dependencies and create /config
# RUN echo "deb http://ftp.debian.org/debian jessie-backports main" >> /etc/apt/sources.list && \
RUN apt-get update && \
    apt-get install -y --no-install-recommends  ffmpeg \
                                                lame \
                                                libopus0 \
                                                libssl-dev \
                                                vorbis-tools \
                                                && \
    mkdir /config


# Create empty shell project
RUN USER=root cargo new --bin punk_bot

WORKDIR /punk_bot

# Copy manifest
COPY ./Cargo.toml ./Cargo.toml

# Build dependencies
RUN cargo build --release
RUN rm src/*.rs

# Set log level
ENV RUST_LOG warn

# Copy run script
COPY src/run /bin

# Copy source tree
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/punk_bot*
RUN cargo build --release

# Copy executable
RUN mv ./target/release/punk_bot /bin && \
    rm -rf /punk_bot

WORKDIR /

# EXPOSE 8080
VOLUME /config
CMD ["run"]
