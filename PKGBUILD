# PKGBUILD
pkgname=my_volume_control
pkgver=0.1.0
pkgrel=1
pkgdesc="Volume control program for Arch Linux"
arch=('x86_64')
url="https://github.com/your_username/my_volume_control"
license=('MIT')
depends=('evdev')

source=("$pkgname-$pkgver.tar.gz::$url/archive/$pkgver.tar.gz")

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release
}

package() {
    cd "$srcdir/$pkgname-$pkgver/target/release"
    install -Dm755 my_volume_control "$pkgdir/usr/bin/my_volume_control"
}
