#!/bin/bash

read -p "Current version: " version
rm -rf ~/.tmp/sherlock-pomodoro-release/
mkdir -p ~/.tmp/sherlock-pomodoro-release/
cargo build --release
cp target/release/sherlock-pomodoro ~/.tmp/sherlock-pomodoro-release/
cp LICENSE ~/.tmp/sherlock-pomodoro-release/LICENSE

cd ~/.tmp/sherlock-pomodoro-release/
tar -czf sherlock-pomodoro-v${version}-bin-linux-x86_64.tar.gz sherlock-pomodoro LICENSE


