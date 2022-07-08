===========================================
Rust bindings for libraries in alsa-gobject
===========================================

2022/07/07
Takashi Sakamoto

Introduction
============

* This repository includes FFI and API bindings for libraries in alsa-gobject project.

  * `<https://github.com/alsa-project/alsa-gobject/>`_

* The crates are available in `crates.io <https://crates.io/>`_ as well.

* The latest release is version 0.0.92. This is pre-release to publish crates in crates.io.

License
=======

MIT License

Examples
========

* ``alsactl/examples/dump-elem-data.rs``

  * Dump the data of control element which belongs to sound card

* ``alsaseq/examples/dump-event-data.rs``

  * Dump the note/ctl data of event received by user client of ALSA Sequencer::
