====================
alsa-gobject Rust bindings
====================

2020/06/21
Takashi Sakamoto

Introduction
============

* This repository includes FFI and API bindings for libraries in alsa-gobject project.

  * https://github.com/alsa-project/alsa-gobject/

License
=======

MIT License

Sample code
===========

* ``alsactl/examples/dump-elem-data.rs``

  * Dump the data of control element which belongs to sound card

* ``alsaseq/examples/dump-event-data.rs``

  * Dump the note/ctl data of event received by user client of ALSA Sequencer::

end
