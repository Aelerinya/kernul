name = kernul
dir := $(PWD)/target/x86_kernul/debug
bin := $(dir)/$(name)
dep := $(dir)/$(name).d
iso := $(name).iso
grub_cfg := grub.cfg
iso_build_dir := .iso

all: $(iso)
	echo $(dep)

-include $(dep)

$(iso_build_dir)/boot/grub:
	mkdir -p $@

$(iso): $(bin) $(grub_cfg) | $(iso_build_dir)/boot/grub
	cp -f $(bin) $(iso_build_dir)/boot/
	cp -f $(grub_cfg) $(iso_build_dir)/boot/grub/
	grub-mkrescue -o $(iso) $(iso_build_dir)

$(bin):
	cargo xbuild

run: all
	qemu-system-x86_64 -cdrom $(iso)
