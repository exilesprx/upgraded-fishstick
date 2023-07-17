FROM rust:1.71.0-bullseye

RUN curl -q 'https://proget.makedeb.org/debian-feeds/prebuilt-mpr.pub' | gpg --dearmor | tee /usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg 1> /dev/null \
  && echo "deb [signed-by=/usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg] https://proget.makedeb.org prebuilt-mpr bullseye" | tee /etc/apt/sources.list.d/prebuilt-mpr.list \
  && apt -y update \
  && apt -y install just avr-libc gcc-avr avrdude libudev-dev build-essential pkg-config

RUN rustup override set nightly-2022-07-10 \
  && rustup component add rust-src --toolchain nightly-2022-07-10-x86_64-unknown-linux-gnu