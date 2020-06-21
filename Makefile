#!/usr/bin/make -f

GIR_EXEC = gir/target/release/gir

.PHONY: all clean

all: alsactl alsatimer alsaseq alsahwdep

clean:
	rm -rf gir-files/ALSACtl-0.0.gir
	rm -rf gir-files/ALSATimer-0.0.gir
	rm -rf gir-files/ALSASeq-0.0.gir
	rm -rf gir-files/ALSAHwdep-0.0.gir
	rm -rf alsactl-sys
	rm -rf alsactl/src/auto alsactl/target alsactl/Cargo.lock
	rm -rf alsatimer-sys
	rm -rf alsatimer/src/auto alsatimer/target alsatimer/Cargo.lock
	rm -rf alsaseq-sys
	rm -rf alsaseq/src/auto alsaseq/target alsaseq/Cargo.lock
	rm -rf alsahwdep-sys

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

alsatimer/src/auto: conf/gir-alsatimer.toml gir-files/ALSATimer-0.0.gir $(GIR_EXEC)
	$(GIR_EXEC) -c conf/gir-alsatimer.toml -d gir-files -m normal -o alsatimer

alsatimer: alsatimer/src/lib.rs alsatimer/Cargo.toml alsatimer-sys alsatimer/src/auto

gir-files/ALSASeq-0.0.gir: ALSASeq-0.0.gir gir-files/GLib-2.0.gir
	cp ALSASeq-0.0.gir gir-files/ALSASeq-0.0.gir

alsaseq-sys/src: conf/gir-alsaseq-sys.toml gir-files/ALSASeq-0.0.gir $(GIR_EXEC)
	$(GIR_EXEC) -c conf/gir-alsaseq-sys.toml -d gir-files -m sys -o alsaseq-sys

alsaseq-sys: alsaseq-sys/src

alsaseq/src/auto: conf/gir-alsaseq.toml gir-files/ALSASeq-0.0.gir $(GIR_EXEC)
	$(GIR_EXEC) -c conf/gir-alsaseq.toml -d gir-files -m normal -o alsaseq

alsaseq: alsaseq/src/lib.rs alsaseq/Cargo.toml alsaseq-sys alsaseq/src/auto

gir-files/ALSAHwdep-0.0.gir: ALSAHwdep-0.0.gir gir-files/GLib-2.0.gir
	cp ALSAHwdep-0.0.gir gir-files/ALSAHwdep-0.0.gir

alsahwdep-sys/src: conf/gir-alsahwdep-sys.toml gir-files/ALSAHwdep-0.0.gir $(GIR_EXEC)
	$(GIR_EXEC) -c conf/gir-alsahwdep-sys.toml -d gir-files -m sys -o alsahwdep-sys

alsahwdep-sys: alsahwdep-sys/src

alsahwdep: alsahwdep-sys
