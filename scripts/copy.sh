#!/bin/sh

udisksctl mount -b /dev/sdb1
rm -f /run/media/scipex/boot/kernel8.img
cp target/aarch64-unknown-none-softfloat/release/kernel8.img /run/media/scipex/boot/kernel8.img
echo "Copied kernel image"
udisksctl unmount -b /dev/sdb1