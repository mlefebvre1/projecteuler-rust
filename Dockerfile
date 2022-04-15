FROM rust:latest

RUN apt-get update && \
    apt-get install -y \
    zsh \
    python3 \
    python3-pip \
    nano

# get Rust components
RUN rustup component add clippy rustfmt
RUN cargo install cargo-tarpaulin

# Install oh-my-zsh
RUN wget https://github.com/robbyrussell/oh-my-zsh/raw/master/tools/install.sh -O - | zsh || true

# Copy and install program
WORKDIR /usr/share/projecteuler-rust
COPY . .
RUN cargo install --path .
CMD ["projecteuler-rust"]
