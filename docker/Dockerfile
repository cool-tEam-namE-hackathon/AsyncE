FROM --platform=linux/amd64 ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive 
RUN apt-get update && apt-get install -y \
    curl \
    qemu-user-static \
    libc6 \
    lib32gcc-s1 \
    libstdc++6 \
    libx32gcc-s1 \
    libx32stdc++6 \
    libunwind8 \
    build-essential \
    ca-certificates \
    libssl-dev \
    pkg-config \
    git \
    nodejs \
    npm

ENV DFXVM_INIT_YES=true
RUN curl --fail --silent --show-error --location --retry 10 --connect-timeout 5 --max-time 20 --retry-connrefused  https://internetcomputer.org/install.sh | bash
ENV PATH="/root/.local/share/dfx/bin:${PATH}"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

RUN cargo update
RUN npm i

EXPOSE 4943

RUN dfx start --background && dfx deploy
