#!/bin/bash

################################################
#             CMDJEWEL BUILD SCRIPT            #
#             ====================             #
# This build script assumes you're on an       #
# amd64 Linux system and uses Podman to make   #
# amd64 Linux and Windows builds.              #
#                                              #
# Hopefully I can build for Mac soon, but I'll #
# need a container to do that!                 #
#                                              #
################################################

# Make build directory
mkdir build/
mkdir build/linux/ -p

# Pull images
podman pull registry.fedoraproject.org/fedora:latest

# 1. Build for Linux amd64
#-----------------------------------------------

podman create --name cmdjewel_linux -v .:/cmdjewel/  --security-opt label=disable --userns=keep-id  -it fedora:latest /bin/bash
podman start cmdjewel_linux

# interactive because... actually idk i am just doing this out of habit.
podman exec -itu root cmdjewel_linux sudo dnf install rust rust-alsa-devel libopenmpt-devel -y

# yes i am building as root do not question my decisions
podman exec -itu root -w cmdjewel cmdjewel_linux cargo build --release

# done!.
cp target/release/cmdjewel build/linux/
