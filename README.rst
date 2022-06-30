==========================
alsa-gobject Rust bindings
==========================

2022/06/30
Takashi Sakamoto

Introduction
============

* This repository includes FFI and API bindings for libraries in alsa-gobject project.

  * https://github.com/alsa-project/alsa-gobject/

The latest release is `version 0.3.0 <https://github.com/alsa-project/alsa-gobject-rs/releases/tag/v0.3.0>`_
which supports `alsa-gobject version 0.3.0 <https://github.com/alsa-project/alsa-gobject/releases/tag/v0.3.0>`_.

License
=======

MIT License

Sample code
===========

* ``alsactl/examples/dump-elem-data.rs``

  * Dump the data of control element which belongs to sound card

* ``alsaseq/examples/dump-event-data.rs``

  * Dump the note/ctl data of event received by user client of ALSA Sequencer::
