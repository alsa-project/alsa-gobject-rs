===========================================
Rust bindings for libraries in alsa-gobject
===========================================

2024/10/06
Takashi Sakamoto

Introduction
============

* This repository includes FFI and API bindings for libraries in alsa-gobject project.

  * `<https://github.com/alsa-project/alsa-gobject/>`_

* The crates are available in `crates.io <https://crates.io/>`_ as well.

* The latest release is version 0.7.0.

Crates
======

API bindings for safe and high-level abstractions
-------------------------------------------------

* `alsactl crate <alsactl/README.md>`_ for alsactl library
* `alsahwdep crate <alsahwdep/README.md>`_ for alsahwdep library
* `alsarawmidi crate <alsarawmidi/README.md>`_ for alsarawmidi library
* `alsatimer crate <alsatimer/README.md>`_ for alsatimer library
* `alsaseq crate <alsaseq/README.md>`_ for alsaseq library

`FFI bindings <https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages>`_
--------------------------------------------------------------------------------------------

* `alsactl-sys crate <alsactl-sys/README.md>`_ for alsactl library
* `alsahwdep-sys crate <alsahwdep-sys/README.md>`_ for alsahwdep library
* `alsarawmidi-sys crate <alsarawmidi-sys/README.md>`_ for alsarawmidi library
* `alsatimer-sys crate <alsatimer-sys/README.md>`_ for alsatimer library
* `alsaseq-sys crate <alsaseq-sys/README.md>`_ for alsaseq library

License
=======

MIT License

Examples
========

* ``alsactl/examples/dump-elem-data.rs``

  * Dump the data of control element which belongs to sound card

* ``alsaseq/examples/dump-event-data.rs``

  * Dump the note/ctl data of event received by user client of ALSA Sequencer::
