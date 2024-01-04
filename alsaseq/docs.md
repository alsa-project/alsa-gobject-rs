<!-- file * -->
<!-- enum ClientType::variant None -->
The client is invalid.
<!-- enum ClientType::variant User -->
The client is userspace application.
<!-- enum ClientType::variant Kernel -->
The client is kernel driver.
<!-- impl Event::fn pointer_data -->
Get the pointer and length in data of event. It has no gurantee that the dereference of pointer
is safe in VMA of user process, thus users have to know the purpose of the pointer in advance;
e.g. inter-process communication between forked parent and child processes.

# Returns

[`true`] when the overall operation finishes successfully, else [`false`].

## `data`
A pointer to user data.

## `length`
The length of user data.
<!-- impl Event::fn set_pointer_data -->
Copy the pointer and length to data of event. When using the type, the event should be deliverd
directly without scheduling to queue. The aim of data pointed by it is arbitrary, thus
programmer needs to decide protocol between transmitter and receiver in advance.
## `data`
A pointer to user data.
## `length`
The length of user data.

# Returns

[`true`] when the overall operation finishes successfully, else [`false`].
<!-- enum EventError::variant Failed -->
General error due to unspecified reason.
<!-- enum EventError::variant InvalidDataType -->
The type of requested data is invalid in the event.
<!-- enum EventError::variant InvalidLengthMode -->
The mode of length for requested data is invalid in
                                             the event.
<!-- enum EventError::variant InvalidTstampMode -->
The type of time stamp for requested data is is
                                             invalid in the event.
<!-- enum EventLengthMode::variant Fixed -->
The data is fixed length.
<!-- enum EventLengthMode::variant Variable -->
The data is variable length.
<!-- enum EventLengthMode::variant Pointer -->
The data is a pointer and its length in userspace.
<!-- enum EventPriorityMode::variant Normal -->
For normal priority.
<!-- enum EventPriorityMode::variant High -->
For high priority.
<!-- enum EventTimeMode::variant Abs -->
The time is absolute.
<!-- enum EventTimeMode::variant Rel -->
The time is relative.
<!-- enum EventTstampMode::variant Tick -->
The time stamp includes tick count.
<!-- enum EventTstampMode::variant Real -->
The time stamp includes real time.
<!-- enum EventType::variant System -->
For system status.
<!-- enum EventType::variant Result -->
For result status.
<!-- enum EventType::variant Note -->
For note message with duration.
<!-- enum EventType::variant Noteon -->
For note on message.
<!-- enum EventType::variant Noteoff -->
For note off message.
<!-- enum EventType::variant Keypress -->
For keypress message.
<!-- enum EventType::variant Controller -->
For control change message.
<!-- enum EventType::variant Pgmchange -->
For program change message.
<!-- enum EventType::variant Chanpress -->
For channel pressure message.
<!-- enum EventType::variant Pitchbend -->
For pitchbend message.
<!-- enum EventType::variant Control14 -->
For control message with 14 bit value.
<!-- enum EventType::variant Nonregparam -->
For 14 bit NRPN address and 14 bit unsigned value.
<!-- enum EventType::variant Regparam -->
For 14 bit RPN address and 14 bit unsigned value.
<!-- enum EventType::variant Songpos -->
For song position message with LSB and MSB values.
<!-- enum EventType::variant Songsel -->
For song select message with numerical ID of song.
<!-- enum EventType::variant Qframe -->
For time code quarter frame message of MIDI.
<!-- enum EventType::variant Timesign -->
For time signature message of Standard MIDi File.
<!-- enum EventType::variant Keysign -->
For key signature message of Standard MIDI File.
<!-- enum EventType::variant Start -->
For Real Time Start message of MIDI.
<!-- enum EventType::variant Continue -->
For Real Time Continue message of MIDI.
<!-- enum EventType::variant Stop -->
For Real Time Stop message of MIDI.
<!-- enum EventType::variant SetposTick -->
For position setting of tick queue.
<!-- enum EventType::variant SetposTime -->
For position setting of realtime queue.
<!-- enum EventType::variant Tempo -->
For tempo message of Standard MIDI File.
<!-- enum EventType::variant Clock -->
For Real Time Clock message of MIDI.
<!-- enum EventType::variant Tick -->
For Real Time Tick message of MIDI.
<!-- enum EventType::variant QueueSkew -->
For skew of tempo for queue.
<!-- enum EventType::variant TuneRequest -->
For requests to tune.
<!-- enum EventType::variant Reset -->
For reset to power-on state.
<!-- enum EventType::variant Sensing -->
For active sensing message.
<!-- enum EventType::variant Echo -->
For echo message.
<!-- enum EventType::variant Oss -->
For raw message from Open Sound System.
<!-- enum EventType::variant ClientStart -->
For appear of the port.
<!-- enum EventType::variant ClientExit -->
For disappear of the client.
<!-- enum EventType::variant ClientChange -->
For change of information or status of the client.
<!-- enum EventType::variant PortStart -->
For addition of the port.
<!-- enum EventType::variant PortExit -->
For removal of the port.
<!-- enum EventType::variant PortChange -->
For change of information or status of the port.
<!-- enum EventType::variant PortSubscribed -->
For establishment of subscription about the port.
<!-- enum EventType::variant PortUnsubscribed -->
For break of subscription about the port.
<!-- enum EventType::variant Usr0 -->
For user-defined message 0.
<!-- enum EventType::variant Usr1 -->
For user-defined message 1.
<!-- enum EventType::variant Usr2 -->
For user-defined message 2.
<!-- enum EventType::variant Usr3 -->
For user-defined message 3.
<!-- enum EventType::variant Usr4 -->
For user-defined message 4.
<!-- enum EventType::variant Usr5 -->
For user-defined message 5.
<!-- enum EventType::variant Usr6 -->
For user-defined message 6.
<!-- enum EventType::variant Usr7 -->
For user-defined message 7.
<!-- enum EventType::variant Usr8 -->
For user-defined message 8.
<!-- enum EventType::variant Usr9 -->
For user-defined message 9.
<!-- enum EventType::variant Sysex -->
For system exclisive message with variable length data.
<!-- enum EventType::variant Bounce -->
For error message.
<!-- enum EventType::variant UsrVar0 -->
For user-defined message 0 with variable length data.
<!-- enum EventType::variant UsrVar1 -->
For user-defined message 1 with variable length data.
<!-- enum EventType::variant UsrVar2 -->
For user-defined message 2 with variable length data.
<!-- enum EventType::variant UsrVar3 -->
For user-defined message 3 with variable length data.
<!-- enum EventType::variant UsrVar4 -->
For user-defined message 4 with variable length data.
<!-- enum EventType::variant None -->
For invalid or unknown message.
<!-- enum QuerySubscribeType::variant Read -->
To query subscribers to read from the port.
<!-- enum QuerySubscribeType::variant Write -->
To query subscribers to write to the port.
<!-- enum QueueTimerType::variant Alsa -->
Any ALSA timer device.
<!-- trait RemoveFilterExt::fn tag -->
The tag of event as filter condition. This is evaluated with
[`RemoveFilterFlag`][crate::RemoveFilterFlag].TAG_MATCH at call of [`UserClientExt::remove_events()`][crate::prelude::UserClientExt::remove_events()].
<!-- trait RemoveFilterExt::fn set_tag -->
The tag of event as filter condition. This is evaluated with
[`RemoveFilterFlag`][crate::RemoveFilterFlag].TAG_MATCH at call of [`UserClientExt::remove_events()`][crate::prelude::UserClientExt::remove_events()].
<!-- enum SpecificAddress::variant Unknown -->
The address for unknown client/port/queue.
<!-- enum SpecificAddress::variant Subscribers -->
The client/port/queue address towards subscribers.
<!-- enum SpecificAddress::variant Broadcast -->
The client/port/queue address to broadcast.
<!-- enum SpecificClientId::variant System -->
The numerical ID to system client.
<!-- enum SpecificClientId::variant Dummy -->
The numerical ID to dummy client.
<!-- enum SpecificClientId::variant Oss -->
The numerical ID to OSS client.
<!-- enum SpecificPortId::variant Timer -->
The numerical ID of port for system timer.
<!-- enum SpecificPortId::variant Announce -->
The numerical ID of port for system announce.
<!-- enum SpecificQueueId::variant Direct -->
The message is delivered immediately, instead of being queued.
<!-- enum UserClientError::variant Failed -->
The system call failed.
<!-- enum UserClientError::variant PortPermission -->
The operation fails due to access permission of port.
<!-- enum UserClientError::variant QueuePermission -->
The operation fails due to access permission of queue.
<!-- enum UserClientError::variant EventUndeliverable -->
The operation failes due to undeliverable event.
