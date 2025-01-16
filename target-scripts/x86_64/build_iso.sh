set -eux
cp -v $LIMINE_DIR/limine-bios.sys $LIMINE_DIR/limine-bios-cd.bin $LIMINE_DIR/limine-uefi-cd.bin $ISO_DIR/boot/limine/
cp -v $LIMINE_DIR/BOOTX64.EFI $ISO_DIR/EFI/BOOT/
cp -v $LIMINE_DIR/BOOTIA32.EFI $ISO_DIR/EFI/BOOT/
xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
	-no-emul-boot -boot-load-size 4 -boot-info-table \
	--efi-boot boot/limine/limine-uefi-cd.bin \
	-efi-boot-part --efi-boot-image --protective-msdos-label \
	$ISO_DIR -o $IMAGE_NAME.iso
./limine/limine bios-install $IMAGE_NAME.iso