FROM alpine:edge

RUN addgroup -S dev && adduser -S -D dev -G dev

RUN apk update && apk add \
    git \
    sudo \
    nano \
    zsh \
    clang \
    gcc \
    g++ \
    cmake \
    musl-dev \
    lld \
    lldb \
    python3 \
    py3-pip \
    curl

USER dev
WORKDIR /home/dev/

# Install oh-my-zsh
RUN wget https://github.com/robbyrussell/oh-my-zsh/raw/master/tools/install.sh -O - | zsh || true

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | zsh -s -- -y
ENV PATH="/home/dev/.cargo/bin:/home/dev/.local/bin:${PATH}"

# COPY . .
# RUN cargo install --path .
# CMD ["projecteuler-rust"]
