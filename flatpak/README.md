### instructions for building:

- `python3 flatpak-generator.py ../Cargo.lock -o cargo-sources.json`
- `flatpak-builder build org.xetibo.ReSet.json --force-clean`
- `flatpak build-export export build`
- `flatpak build-bundle export reset.flatpak org.xetibo.ReSet`
- you can also use the build.sh script provided
- note: if you are using a point release distribution(ubuntu, debian stable etc. please use the flatpak version of these commands -> flatpak run org.flatpak.Builder build...)

### instructions for installation:

`flatpak install --user reset.flatpak`

### permissions
currently ReSet uses permission on all devices, for some reason otherwise it can't access sound settings like volume changes etc.

This can likely be fixed by implementing portal integration later.
