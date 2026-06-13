# Flatpak Guide

Use the Flatpak bundle when you want a sandboxed install that is less dependent on the host distribution layout.

## Install

```sh
flatpak install ./dist/HDHomeRunLinuxPlayer.flatpak
```

## Run

```sh
flatpak run io.github.e88z4.HDHomeRunLinuxPlayer
```

## Uninstall

```sh
flatpak uninstall io.github.e88z4.HDHomeRunLinuxPlayer
```

## Runtime Notes

- the Flatpak package includes both the desktop client and the bundled backend
- the Flatpak package also includes the `hdhr-headless` helper CLI
- packaged playback defaults to embedded Qt Multimedia playback inside the app window
- the Flatpak runtime currently targets KDE `6.10`

## When To Use Flatpak

- you want a reproducible runtime across distributions
- you prefer sandboxed desktop apps
- you already use Flatpak for desktop software on the system
