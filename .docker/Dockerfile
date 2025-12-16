FROM rust:1.92.0-slim

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
