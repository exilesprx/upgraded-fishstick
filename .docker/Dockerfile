FROM rust:1.77.1-bookworm

RUN apt-get -y update \
  && apt-get -y install apt-transport-https avr-libc gcc-avr avrdude libudev-dev build-essential pkg-config

RUN wget -qO - 'https://proget.makedeb.org/debian-feeds/prebuilt-mpr.pub' | gpg --dearmor | tee /usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg 1> /dev/null
RUN echo "deb [arch=all,$(dpkg --print-architecture) signed-by=/usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg] https://proget.makedeb.org prebuilt-mpr bookworm" | tee /etc/apt/sources.list.d/prebuilt-mpr.list
RUN apt-get -y update \
  && apt-get -y install just

RUN rustup override set nightly-2022-07-10 \
  && rustup component add rust-src --toolchain nightly-2022-07-10-x86_64-unknown-linux-gnu