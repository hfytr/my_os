 add-symbol-file /home/fbwdw/docs/os/target/x86_64-unknown-none/debug/deps/artifact/kernel-d051de5109deb413/bin/kernel-d051de5109deb413 -o 0xffff800000000000
target remote :1234
b _start
c