python3 flatpak-generator.py ../Cargo.lock -o cargo-sources.json
flatpak-builder build org.Xetibo.ReSet.json --force-clean
flatpak build-export export build
flatpak build-bundle export reset.flatpak org.Xetibo.ReSet
