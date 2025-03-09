s=root@192.168.64.201
# s=newbeebox@172.16.1.130

build() {
    cargo zigbuild -r --target x86_64-unknown-linux-musl -p wego-devops
}

pub() {
    scp target/x86_64-unknown-linux-musl/release/wego-devops ${s}:/home/devops/
    scp -r web/dist/* ${s}:/home/devops/wwwroot
}

test() {
    ssh -t root@192.168.64.201 "
    set -xe
    export PS4='+[$(date "+%Y-%m-%d %H:%M:%S")] '
    date
    cd /home/devops/cache/projects/wego-server-desktop
    ls -l logs 
    sleep 5
    echo '-----------------------------------'
    cat logs/20250227151613.log
    " | tee -a local_logfile.log
}

test2() {
    PS4='+[$(date "+%Y-%m-%d %H:%M:%S")] ' sh -xe <<EOF 2>&1 | tee -a logfile.log
pwd
ls -l
echo "Script completed"
EOF
}

$@
