#!/usr/bin/make -f

GIR_EXEC = gir/target/release/gir

.PHONY: all clean

all: alsactl alsatimer

clean:
	rm -rf gir-files/ALSACtl-0.0.gir
	rm -rf gir-files/ALSATimer-0.0.gir
	rm -rf alsactl-sys
	rm -rf alsactl/src/auto alsactl/target alsactl/Cargo.lock
	rm -rf alsatimer-sys
	rm -rf alsatimer/src/auto alsatimer/target alsatimer/Cargo.lock

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

alsactl/src/auto: conf/gir-alsactl.toml gir-files/ALSACtl-0.0.gir $(GIR_EXEC)
	$(GIR_EXEC) -c conf/gir-alsactl.toml -d gir-files -m normal -o alsactl

alsactl: alsactl/src/lib.rs alsactl/Cargo.toml alsactl/src/auto alsactl-sys

gir-files/ALSATimer-0.0.gir: ALSATimer-0.0.gir gir-files/GLib-2.0.gir
	cp ALSATimer-0.0.gir gir-files/ALSATimer-0.0.gir

alsatimer-sys/src: conf/gir-alsatimer-sys.toml gir-files/ALSATimer-0.0.gir $(GIR_EXEC)
	$(GIR_EXEC) -c conf/gir-alsatimer-sys.toml -d gir-files -m sys -o alsatimer-sys

alsatimer-sys: alsatimer-sys/src

alsatimer: alsatimer-sys
