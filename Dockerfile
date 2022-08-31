FROM debian:stretch-slim AS build-env

# Run with `docker build --build-arg dune_profile=<profile_name>` ...
ARG user_id=50000
ARG dune_profile=mainnet
ARG mina_git="https://github.com/name-placeholder/mina.git"
ARG mina_branch="decoder-fuzzing-ipc"

USER root

ENV DUNE_PROFILE $dune_profile
ENV DEBIAN_FRONTEND noninteractive
ENV PATH=$PATH:/usr/local/go/bin
ENV PATH=$PATH:/home/mina/.cargo/bin
ENV PATH=$PATH:/home/mina/bin

ENV MINA_LIBP2P_HELPER_PATH=/home/mina/bin/libp2p_helper

# Dependencies
RUN apt-get -y update && \
  apt-get -y upgrade && \
  apt-get -y install \
    apt-transport-https \
    ca-certificates \
    pkg-config \
    build-essential \
    curl \
    git \
    dnsutils \
    dumb-init \
    gettext \
    gnupg2 \
    unzip \
    bubblewrap \
    jq \
    libgmp10 \
    libgomp1 \
    libssl1.1 \
    libpq-dev \
    libffi-dev \
    libgmp-dev \
    libssl-dev \
    libbz2-dev \
    zlib1g-dev \
    m4 \
    libsodium-dev \
    libjemalloc-dev \
    procps \
    python3 \
    tzdata && \
  rm -rf /var/lib/apt/lists/*

# Go
RUN curl -sSL https://go.dev/dl/go1.18.3.linux-amd64.tar.gz | tar -C /usr/local -xzf -

# Capnproto
RUN curl -sSL https://capnproto.org/capnproto-c++-0.10.2.tar.gz | tar -zxf - \
  && cd capnproto-c++-0.10.2 \
  && ./configure \
  && make -j6 check \
  && make install

# Opam
RUN bash -c "echo -ne '\n\n\n\n' | sh <(curl -fsSL https://raw.githubusercontent.com/ocaml/opam/master/shell/install.sh)"

RUN mkdir /home/mina \
  && adduser --uid ${user_id} --disabled-password --gecos '' mina \
  && passwd -l mina \
  && chown -R mina:mina /home/mina

WORKDIR /home/mina
USER mina

# Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Go support for Capnproto
RUN go install capnproto.org/go/capnp/v3/capnpc-go@v3.0.0-alpha.1

RUN git clone ${mina_git} --branch ${mina_branch} --depth=1 ./src \
  && cd src \
  && git submodule update --init --recursive \
  && git config --local --add submodule.recurse true

RUN cd src \
  && opam init --disable-sandboxing \
  && opam switch import -y src/opam.export \
  && ./scripts/pin-external-packages.sh

RUN mkdir ~/bin \
  && cd src \
  && eval $(opam config env) \
  && make gossip_net_decoder \
  && cp _build/default/src/lib/gossip_net/decoder/decoder.exe ~/bin/gossip_net_decoder \
  && rm -rf _build

ENTRYPOINT [ "gossip_net_decoder" ]
