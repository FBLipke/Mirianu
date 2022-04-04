#!/bin/bash
cargo doc --no-deps --document-private-items
cp -r /usr/local/Projekte/Mirianu/target/x86_64-mirianu_os/doc /var/www/html/
