# <center>ReSet</center>

<center>

![Logo of ReSet](./assets/ReSet.png)

</center>

<center>A window manager/compositor agnostic settings application for Linux written in rust and gtk4.</center>



## Features
- Bluetooth via bluez
- Audio via PulseAudio
- Wi-Fi via NetworkManager

## Screenshots

<center>

### Audio

<img alt="Audio Screenshot of ReSet" src="./assets/reset_audio.png"  width="80%">

### Wi-Fi
<img alt="Wi-Fi Screenshot of ReSet" src="./assets/reset_wifi.png"  width="80%">

### Bluetooth
<img alt="Bluetooth Screenshot of ReSet" src="./assets/reset_bluetooth.png"  width="80%">
</center>

## Packaging
ReSet is available with the following packaging solutions:

### Flatpak
We are currently not published on flatpak due to issues with permissions.
This is being worked on...

installation:
Download the flatpak package from the release and install with the terminal.
```
flatpak install --user reset.flatpak
```

### Arch Package
<!-- AUR: -->
<!-- ```paru -S ReSet``` -->

Manually:
Download the package from the releases tab and install it with pacman.

```sudo pacman -U /path/to/reset```

### Debian Package(Ubuntu 23.04 dependencies)

Download the package from the releases tab and install it with apt.
```sudo apt install ./path/to/reset```
### crates
```cargo install ReSet```
### Compiled Binary

The compiled binary is provided in the releases.
## Roadmap

This application was developed as a semester project for the Eastern Switzerland University of Applied Sciences.
With potential advancements as a next project, due to this, no major development will happen until February 2024.
However, there is still a roadmap for this application.

- Plugin System
- Accessibility Features
- Better Error handling
- Customizable shortcuts
- and more
