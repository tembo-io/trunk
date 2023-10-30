ARG PG_VERSION=15
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.10.2
# Switch to the root user for installing dependencies and building the extension
USER root
# Update the package list and install the necessary build dependencies
# Extension build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    libreadline-dev \
    zlib1g-dev \
    flex \
    bison \
    libxml2-dev \
    libxslt-dev \
    libssl-dev \
    libxml2-utils \
    xsltproc \
    ccache \
    python3-dev \
    clang \
    cmake \
    libclang-dev \
    libopenblas-dev \
    libpython3-dev \
    pkg-config \
    crossbuild-essential-arm64 \
    crossbuild-essential-amd64 \
    gcc-aarch64-linux-gnu \
    gfortran && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
# Install Rust using the official installation script
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# Set the default Rust version to stable
RUN rustup default stable
# Add necessary Rust components and target
RUN rustup component add llvm-tools-preview rustc-dev && \
    rustup target install x86_64-unknown-linux-gnu
# Clone the repository into /app/pg_linearalgebra
RUN git clone https://github.com/joshuajerin/pg_linearalgebra.git /app/pg_linearalgebra
# Set the working directory to /app/pg_linearalgebra
WORKDIR /app/pg_linearalgebra
# Build the extension using cargo pgrx
RUN cargo pgrx init --pg15 /usr/bin/pg_config && \
    cargo pgrx package -c /usr/bin/pg_config