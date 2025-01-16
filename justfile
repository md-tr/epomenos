alias bb := build_binary
alias bi := build_iso
alias rb := run_bios

# target-triple to be used
target := "x86_64"

# cargo profile to be used
profile := "dev"

# image name to be used
image_name := "epome"

# iso directory to be created and used
iso_dir := "epome-iso"

# limine directory for limine src to be installed
limine_dir := "limine"

# ovmf directory for ovmf fd files to be installed
ovmf_dir := "ovmf"

# additional qemu flags to be used
qemu_flags := "-m 2G -serial stdio --no-reboot"

# the directory name for the binary, regarding profile (DO NOT MODIFY!)
profile_subdir := if profile == "dev" {
    "debug"
} else {
    "release"
}

# the directory where the kernel binary is saved (DO NOT MODIFY!)
kernel_dir := "target" / target / profile_subdir

# print this help message
@default:
    just --list

# install limine
install_limine:
    #!/usr/bin/env sh
    set -eux
    if [ ! -d "{{limine_dir}}" ]; then
        git clone https://github.com/limine-bootloader/limine.git {{limine_dir}} --branch=v8.x-binary --depth=1
        make limine --directory={{limine_dir}}
        echo "{{BOLD + GREEN}}Installed!{{NORMAL}}"
    else
        echo "limine already installed. run 'rm -rf {{limine_dir}}' first if you want it to be reinstalled."
    fi

install_ovmf:
    #!/usr/bin/env sh
    set -eux
    if [ ! -d "{{ovmf_dir}}" ]; then
        mkdir -p ovmf
        curl -Lo {{ovmf_dir}}/ovmf-code-{{target}}.fd https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/ovmf-code-{{target}}.fd
        curl -Lo {{ovmf_dir}}/ovmf-vars-{{target}}.fd https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/ovmf-vars-{{target}}.fd
    else
        echo "ovmf already installed. run 'rm -rf {{ovmf_dir}}' first if you want it to be reinstalled."
    fi

# list available targets
@target_list:
    find "./target-specs" -maxdepth 1 -type f -iname "*.json" -execdir basename {} .json ';'

# clean
@clean:
    rm -rf {{iso_dir}} {{image_name}}.iso {{image_name}}.hdd
    cargo clean

# build the kernel binary
[group('build')]
build_binary:
    #!/usr/bin/env sh
    cargo build --target "target-specs/{{target}}.json" \
                --profile {{profile}} \
                -Zbuild-std=core,compiler_builtins \
                -Zbuild-std-features=compiler-builtins-mem

# build kernel iso from the binary
[group('build')]
build_iso: install_limine build_binary
    #!/usr/bin/env sh
    set -eux
    rm -rf {{iso_dir}}
    mkdir -p {{iso_dir}}/boot/
    cp -v {{kernel_dir}}/kernel {{iso_dir}}/boot/
    mkdir -p {{iso_dir}}/boot/limine
    cp -v limine.conf {{iso_dir}}/boot/limine/
    mkdir -p {{iso_dir}}/EFI/BOOT
    env ISO_DIR="{{iso_dir}}" LIMINE_DIR="{{limine_dir}}" \
        IMAGE_NAME="{{image_name}}" sh "target-scripts/{{target}}/build_iso.sh"
    ls -R {{iso_dir}}
    rm -rf {{iso_dir}}

[group('run')]
run_bios: build_iso
    qemu-system-{{target}} \
        -M q35 \
        -cdrom {{image_name}}.iso \
        -boot d \
        {{qemu_flags}}

[group('debug')]
debug_bios: build_iso
    qemu-system-{{target}} \
        -M q35 \
        -cdrom {{image_name}}.iso \
        -boot d \
        -s -S \
        {{qemu_flags}}

[group('run')]
run_x86_64: install_ovmf build_iso
    qemu-system-{{target}} \
        -M q35 \
        -drive if=pflash,unit=0,format=raw,file={{ovmf_dir}}/ovmf-code-{{target}}.fd,readonly=on \
        -drive if=pflash,unit=1,format=raw,file={{ovmf_dir}}/ovmf-vars-{{target}}.fd \
        -cdrom {{image_name}}.iso \
        {{qemu_flags}}