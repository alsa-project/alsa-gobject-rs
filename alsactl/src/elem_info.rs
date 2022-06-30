// SPDX-License-Identifier: MIT
use super::*;

pub enum ElemInfo {
    Iec60958(ElemInfoIec60958),
    Boolean(ElemInfoBoolean),
    Bytes(ElemInfoBytes),
    Integer(ElemInfoInteger),
    Integer64(ElemInfoInteger64),
    Enumerated(ElemInfoEnumerated),
}
