# AppImage Guide

The AppImage package is the easiest portable distribution option for HDHomeRun Linux Player.

## Run

```sh
chmod +x dist/HDHomeRunLinuxPlayer-x86_64.AppImage
./dist/HDHomeRunLinuxPlayer-x86_64.AppImage
```

## What Is Bundled

- the desktop client
- the local backend
- the runtime helper used to point the client at the packaged backend

The AppImage package uses embedded in-window playback by default. No host `mpv` install is required for normal packaged use.

## Update Strategy

Replace the old AppImage file with the new one and launch it again.

## When To Use AppImage

- you want a portable single-file package
- you do not want a system-level install step
- you want to try the app quickly on a compatible Linux desktop
