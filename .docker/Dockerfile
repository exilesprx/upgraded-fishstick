FROM rust:1.91.0-bookworm

RUN apt-get -y update \
  && apt-get -y install \
  apt-transport-https \
  avr-libc \
  gcc-avr \
  binutils-avr \
  avrdude \
  libudev-dev \
  build-essential \
  pkg-config
