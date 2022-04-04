#!/bin/bash
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-mirianu_os/debug/bootimage-mirianu_os.bin -curses
