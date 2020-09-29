FROM ruby:2.7

ARG DEBIAN_FRONTEND=noninteractive

RUN mv /bin/sh /bin/sh_tmp && ln -s /bin/bash /bin/sh

RUN apt-get update && \
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
  source $HOME/.cargo/env

ENV CARGO_TARGET_DIR /tmp/target
ENV LD_LIBRARY_PATH /usr/local/lib
ENV USER root

VOLUME /tmp/target
VOLUME /usr/local/cargo/registry