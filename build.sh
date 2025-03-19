#!/bin/bash

################################################
#             CMDJEWEL BUILD SCRIPT            #
#             ====================             #
# This build script assumes you're on an       #
# amd64 Linux system and uses Podman to make   #
# amd64 Linux and Windows builds.              #
#                                              #
# Hopefully I can build for Mac soon!          #
#                                              #
################################################

OPENMPT_LINK="https://builds.openmpt.org/builds/auto/libopenmpt/dev.windows.vs2022/0.8.0-pre.14/libopenmpt-0.8.0-pre.14+r23059.dev.windows.vs2022.7z"

# Make build directory
mkdir build/
mkdir build/linux/ -p
mkdir build/windows/ -p

# Pull images
podman pull registry.fedoraproject.org/fedora:latest
podman pull mcr.microsoft.com/windows:ltsc2019

# 1. Build for Linux amd64
#-----------------------------------------------

podman create --name cmdjewel_build -v .:/cmdjewel/  --security-opt label=disable --userns=keep-id  -it fedora:latest /bin/bash
podman start cmdjewel_build

# interactive because... actually idk i am just doing this out of habit.
podman exec -itu root cmdjewel_build dnf install rust rust-alsa-devel libopenmpt-devel rust-std-static-x86_64-pc-windows-gnu git wget p7zip -y

# yes i am building as root do not question my decisions
podman exec -itu root -w cmdjewel cmdjewel_build cargo build --release

# done!.
cp target/release/cmdjewel build/linux/

# 2. Build for Windows amd64 (cross-compile)
#-----------------------------------------------

# Install libopenmpt
podman exec -itu root cmdjewel_build mkdir -p /openmpt
podman exec -itu root -w openmpt cmdjewel_build wget $OPENMPT_LINK
podman exec -itu root -w openmpt cmdjewel_build 7za x libopenmpt*.7z

# Build for Windows
podman exec -itu root -w cmdjewel cmdjewel_build cargo install cross
podman exec -itu root -w cmdjewel --env RUSTFLAGS="-L /openmpt/bin/amd64" cmdjewel_build cargo build --release --target x86_64-pc-windows-gnu

# Done!
cp target/x86_64-pc-windows-gnu/release/cmdjewel.exe build/windows/cmdjewel.exe
podman exec -itu root cmdjewel_build cp find /openmpt/bin/amd64/ -type f -exec cp {} /cmdjewel/build/windows/ \;
