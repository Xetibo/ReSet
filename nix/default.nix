{ rustPlatform
, rust-bin
, pulseaudio
, dbus
, gdk-pixbuf
, gnome
, pkg-config
, wrapGAppsHook4
, gtk4
, libadwaita
, python312Packages
, flatpak
, flatpak-builder
, lib
, lockFile
, ...
}:
let
  cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
in
rustPlatform.buildRustPackage rec {
  pname = cargoToml.package.name;
  version = cargoToml.package.version;

  src = ../.;

  buildInputs = [
    gtk4
    libadwaita
    pulseaudio
    dbus
    gdk-pixbuf
    gnome.adwaita-icon-theme
    python312Packages.aiohttp
    python312Packages.toml
    flatpak
    flatpak-builder
  ];

  cargoLock = {
    inherit lockFile;
  };

  nativeBuildInputs = [
    pkg-config
    wrapGAppsHook4
    rust-bin.nightly."2024-05-10".default
  ];

  copyLibs = true;

  postInstall = ''
    	install -D --mode=444 $src/${pname}.desktop $out/share/applications/${pname}.desktop
    	install -D --mode=444 $src/src/resources/icons/ReSet.svg $out/share/pixmaps/ReSet.svg
  '';

  # test is broken in nix for some reason
  doInstallCheck = false;
  doCheck = false;

  meta = with lib; {
    description = "A wip universal Linux settings application.";
    homepage = "https://github.com/Xetibo/ReSet";
    changelog = "https://github.com/Xetibo/ReSet/releases/tag/${version}";
    license = licenses.gpl3;
    maintainers = with maintainers; [ DashieTM ];
    mainProgram = "reset";
  };
}
