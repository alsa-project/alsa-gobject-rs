<!-- file * -->
<!-- enum CardError::variant Failed -->
The system call failed.
<!-- enum CardError::variant Disconnected -->
The card associated to the instance is in disconnect state.
<!-- enum CardError::variant ElemNotFound -->
The control element not found in the card.
<!-- enum CardError::variant ElemNotSupported -->
The operation is not supported by the control element.
<!-- enum CardError::variant ElemOwned -->
The control element is owned by the other process.
<!-- enum CardError::variant ElemExist -->
The control element already exists.
<!-- enum ElemIfaceType::variant Card -->
The element has effects to whole the sound card.
<!-- enum ElemIfaceType::variant Hwdep -->
The element has effects to hwdep device.
<!-- enum ElemIfaceType::variant Mixer -->
The element has effects to mixer device.
<!-- enum ElemIfaceType::variant Pcm -->
The element has effects to PCM device.
<!-- enum ElemIfaceType::variant Rawmidi -->
The element has effects to Rawmidi device.
<!-- enum ElemIfaceType::variant Timer -->
The element has effects to Timer device.
<!-- enum ElemIfaceType::variant Sequencer -->
The element has effects to Sequencer device.
<!-- enum ElemType::variant None -->
Unudentified type.
<!-- enum ElemType::variant Boolean -->
The element has boolean values.
<!-- enum ElemType::variant Integer -->
The element has integer values.
<!-- enum ElemType::variant Enumerated -->
The element has values for enumerated labels.
<!-- enum ElemType::variant Bytes -->
The element has byte values.
<!-- enum ElemType::variant Iec60958 -->
The element has parameters of IEC 60958.
<!-- enum ElemType::variant Integer64 -->
The element has 64 bit integer values.
<!-- trait ElemValueExtManual::fn bool -->
Refer to the array specific to [`ElemType`][crate::ElemType].BOOLEAN element in internal storage.

# Returns


## `values`
The array for boolean values.
<!-- trait ElemValueExtManual::fn enum_ -->
Refer to the array specific to [`ElemType`][crate::ElemType].ENUMERATED element in internal storage.

# Returns


## `values`
The array for enumeration index
         values.
<!-- enum EventType::variant Elem -->
The event is related to any element.
