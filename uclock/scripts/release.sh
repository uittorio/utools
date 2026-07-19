#!/usr/bin/env bash

# build and copy release to bin folder so it can be executed by the systemd service
cargo build --release && cp target/release/uclock ~/.local/bin/

# copy service file so it will autostart on login
mkdir -p ~/.config/systemd/user && cp scripts/uclock.service ~/.config/systemd/user/uclock.service

# start service
systemctl --user daemon-reload
systemctl --user enable uclock.service
systemctl --user start uclock.service
