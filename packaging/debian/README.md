# Debian Package Guide

Use the Debian package when you want the app installed through the normal Debian package workflow.

## Install

```sh
sudo apt install ./dist/hdhomerun-linux-player_0.2.0_amd64.deb
```

## Run

```sh
hdhomerun-linux-player
```

## Remove

```sh
sudo apt remove hdhomerun-linux-player
```

## Package Behavior

- installs the desktop client and backend under the normal system package layout
- relies on host Qt runtime packages instead of bundling a separate `mpv` runtime
- uses embedded in-window playback by default

## When To Use Debian Packaging

- you are on Debian or a Debian-derived distribution
- you prefer package-manager installs and removals
- you want desktop integration through the normal distro package path
