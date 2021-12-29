# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.163.1/containers/rust/.devcontainer/base.Dockerfile
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends build-essential

# RUN cargo install cargo-compete --locked
RUN wget https://github.com/qryxip/cargo-compete/releases/download/v0.10.2/cargo-compete-v0.10.2-x86_64-unknown-linux-gnu.tar.gz; \
    tar -zxvf cargo-compete-v0.10.2-x86_64-unknown-linux-gnu.tar.gz; \
    cd cargo-compete-v0.10.2-x86_64-unknown-linux-gnu; \
    cp cargo-compete /usr/local/bin/;
