.PHONY: all clean run

all: dist/system.iso
	
clean:
	rm -rf dist/*

run: dist/system.iso
	qemu-system-x86_64 $< 

dist/system.iso: dist/root/
	grub-mkrescue -o $@ $<

dist/root/: dist/system.bin
	mkdir -p $@
	mkdir -p $@boot
	cp $< $@boot
	mkdir -p $@boot/grub
	cp grub.cfg $@boot/grub

dist/system.bin: dist/boot.o dist/kernel.o
	ld -T linker.ld -o $@ $^

dist/boot.o: src/boot.s
	nasm -felf64 -o $@ $^

dist/kernel.o: src/kernel.rs
	rustc --target=x86_64-unknown-none --emit=obj -o $@ $^
