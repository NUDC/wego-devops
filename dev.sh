s=root@192.168.64.201

build() {
    cargo zigbuild -r --target x86_64-unknown-linux-musl
}

pub() {
    scp target/x86_64-unknown-linux-musl/release/wego-devops ${s}:/home/wego/devops/
}
$@
