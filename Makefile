#!/usr/bin/make -f

GIR_EXEC = gir/target/release/gir

.PHONY: all clean

all: alsactl

clean:
	rm -rf gir-files/ALSACtl-0.0.gir
	rm -rf alsactl-sys

gir/Cargo.toml:
	git submodule update --init gir

$(GIR_EXEC): gir/Cargo.toml
	cd gir && cargo build --release

gir-files/GLib-2.0.gir:
	git submodule update --init gir-files

gir-files/ALSACtl-0.0.gir: ALSACtl-0.0.gir gir-files/GLib-2.0.gir
	cp ALSACtl-0.0.gir gir-files/ALSACtl-0.0.gir

alsactl-sys/src: conf/gir-alsactl-sys.toml gir-files/ALSACtl-0.0.gir $(GIR_EXEC)
	$(GIR_EXEC) -c conf/gir-alsactl-sys.toml -d gir-files -m sys -o alsactl-sys

alsactl-sys: alsactl-sys/src

alsactl: alsactl-sys
