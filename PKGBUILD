# Original made by: VorpalBlade <VorpalBlade@users.noreply.github.com>
#
# Maintainer: Pokume Kachi <108186213+PokumeKachi@users.noreply.github.com>
pkgname=konfigkoll-local
_pkgname=${pkgname%-local}
pkgver=0.1.18
pkgrel=1
pkgdesc="Personal system configuration manager for Arch Linux (local build)"
arch=(x86_64 i686 armv7h aarch64)
url="https://github.com/PokumeKachi/paketkoll"
license=('MPL-2.0')
makedepends=('cargo' 'cmake' 'clang')
options=('!lto') # LTO breaks with ring
provides=("${_pkgname}")
conflicts=("${_pkgname}")

source=()
sha256sums=()

prepare() {
    cd "$startdir"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    cd "$startdir"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    make CC=clang CXX=clang++ \
        CARGO_FLAGS='--frozen --no-default-features --features=arch_linux -p konfigkoll -p xtask'
}

check() {
    cd "$startdir"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    make test CC=clang CXX=clang++ \
        CARGO_FLAGS='--frozen --no-default-features --features=arch_linux -p konfigkoll -p xtask'
}

package() {
    cd "$startdir"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    make install-konfigkoll CC=clang CXX=clang++ \
        DESTDIR="$pkgdir" PREFIX=/usr \
        CARGO_FLAGS='--frozen --no-default-features --features=arch_linux -p konfigkoll -p xtask'
}
