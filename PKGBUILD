# Maintainer: Fabio Lenherr <dashie@dashie.org>

pkgname=ReSet
pkgver=1.2.5
pkgrel=0
arch=('x86_64')
pkgdir="/usr/bin/${pkgname}"
pkgdesc="A wip universal Linux settings application."
depends=('rust' 'gtk4' 'dbus')

build() {
	cargo build --release
}

package() {
	cd ..
	install -Dm755 target/release/"$pkgname" "$pkgdir"/usr/bin/"$pkgname"
	install -Dm644 "$pkgname.desktop" "$pkgdir/usr/share/applications/$pkgname.desktop"
	install -Dm644 "src/resources/icons/$pkgname.svg" "$pkgdir/usr/share/pixmaps/$pkgname.svg"
}
