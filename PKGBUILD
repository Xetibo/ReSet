# Maintainer: Fabio Lenherr <dashie@dashie.org>

pkgname=ReSet
pkgver=2.0.0
pkgrel=0
arch=('x86_64')
pkgdir="/usr/bin/${pkgname}"
pkgdesc="A wip universal Linux settings application."
depends=('gtk4' 'dbus' 'libadwaita')
optdepends=('pipewire-pulse' 'networkmanager' 'bluez')
makedepends=('rust')


build() {
	cargo build --release
}

package() {
	cd ..
	install -Dm755 target/release/"$pkgname" "$pkgdir"/usr/bin/"$pkgname"
	install -Dm644 "$pkgname.desktop" "$pkgdir/usr/share/applications/$pkgname.desktop"
	install -Dm644 "src/resources/icons/$pkgname.svg" "$pkgdir/usr/share/pixmaps/$pkgname.svg"
}
