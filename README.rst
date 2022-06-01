====================
alsa-gobject Rust bindings
====================

2022/03/17
Takashi Sakamoto

Introduction
============

* This repository includes FFI and API bindings for libraries in alsa-gobject project.

  * `<https://github.com/alsa-project/alsa-gobject/>`_

License
=======

MIT License

Dependencies
============

* Rust version 1.57 or later (edition 2021)
* `alsa-gobject <https://github.com/alsa-project/alsa-gobject/>`_
* FFI crates (``alsactl-sys``, ``alsahwdep-sys``, ``alsarawmidi-sys``)

  * ``libc`` >= 0.2
  * ``glib-sys`` >= 0.15
  * ``gobject-sys`` >= 0.15

* API crates (``alsactl``, ``alsahwdep``)

  * ``libc`` >= 0.2
  * ``bitflags`` >= 1.0
  * ``glib`` >= 0.15
  * FFI crate per each (``alsactl-sys``, ``alsahwdep-sys``)

Sample code
===========

* ``alsactl/examples/dump-elem-data.rs``

  * Dump the data of control element which belongs to sound card

* ``alsaseq/examples/dump-event-data.rs``

  * Dump the note/ctl data of event received by user client of ALSA Sequencer::


How to generate FFI and API crates
==================================

::

    $ ./generator.py

end
