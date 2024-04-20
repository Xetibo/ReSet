{ pkgs ? import <nixpkgs> { } }:

with pkgs;
mkShell {
  nativeBuildInputs = [
    pkg-config
    wrapGAppsHook4
  ];

  buildInputs = [
    dbus
    pulseaudio
    gnome.adwaita-icon-theme
    gtk4
    libadwaita
    gdk-pixbuf
  ];

}
