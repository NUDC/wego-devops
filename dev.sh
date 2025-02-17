s=root@192.168.64.201

build() {
    cargo zigbuild -r --target x86_64-unknown-linux-musl
}

pub_server() {
    scp target/x86_64-unknown-linux-musl/release/wego-devops ${s}:/home/wego/devops/
}
pub_web() {
    scp -r web/dist/* ${s}:/home/wego/devops/wwwroot
}

pub() {
    pub_web
    pub_server
}
$@
