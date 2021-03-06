// automatically generated by the FlatBuffers compiler, do not modify

use std::cmp::Ordering;
use std::mem;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum TriggerSource {
    TimestampReset = 0,
    ExternalSignalRisingEdge = 1,
    ExternalSignalFallingEdge = 2,
    ExternalSignalPulse = 3,
    ExternalGeneratorRisingEdge = 4,
    ExternalGeneratorFallingEdge = 5,
    FrameBegin = 6,
    FrameEnd = 7,
    ExposureBegin = 8,
    ExposusreEnd = 9,
}

const ENUM_MIN_TRIGGER_SOURCE: i8 = 0;
const ENUM_MAX_TRIGGER_SOURCE: i8 = 9;

impl<'a> flatbuffers::Follow<'a> for TriggerSource {
    type Inner = Self;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        flatbuffers::read_scalar_at::<Self>(buf, loc)
    }
}

impl flatbuffers::EndianScalar for TriggerSource {
    #[inline]
    fn to_little_endian(self) -> Self {
        let n = i8::to_le(self as i8);
        let p = &n as *const i8 as *const TriggerSource;
        unsafe { *p }
    }
    #[inline]
    fn from_little_endian(self) -> Self {
        let n = i8::from_le(self as i8);
        let p = &n as *const i8 as *const TriggerSource;
        unsafe { *p }
    }
}

impl flatbuffers::Push for TriggerSource {
    type Output = TriggerSource;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<TriggerSource>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_TRIGGER_SOURCE: [TriggerSource; 10] = [
    TriggerSource::TimestampReset,
    TriggerSource::ExternalSignalRisingEdge,
    TriggerSource::ExternalSignalFallingEdge,
    TriggerSource::ExternalSignalPulse,
    TriggerSource::ExternalGeneratorRisingEdge,
    TriggerSource::ExternalGeneratorFallingEdge,
    TriggerSource::FrameBegin,
    TriggerSource::FrameEnd,
    TriggerSource::ExposureBegin,
    TriggerSource::ExposusreEnd,
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_TRIGGER_SOURCE: [&'static str; 10] = [
    "TimestampReset",
    "ExternalSignalRisingEdge",
    "ExternalSignalFallingEdge",
    "ExternalSignalPulse",
    "ExternalGeneratorRisingEdge",
    "ExternalGeneratorFallingEdge",
    "FrameBegin",
    "FrameEnd",
    "ExposureBegin",
    "ExposusreEnd",
];

pub fn enum_name_trigger_source(e: TriggerSource) -> &'static str {
    let index = e as i8;
    ENUM_NAMES_TRIGGER_SOURCE[index as usize]
}

pub enum TriggerOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Trigger<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Trigger<'a> {
    type Inner = Trigger<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Trigger<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Trigger { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TriggerArgs,
    ) -> flatbuffers::WIPOffset<Trigger<'bldr>> {
        let mut builder = TriggerBuilder::new(_fbb);
        builder.add_t(args.t);
        builder.add_source(args.source);
        builder.finish()
    }

    pub const VT_T: flatbuffers::VOffsetT = 4;
    pub const VT_SOURCE: flatbuffers::VOffsetT = 6;

    #[inline]
    pub fn t(&self) -> i64 {
        self._tab.get::<i64>(Trigger::VT_T, Some(0)).unwrap()
    }
    #[inline]
    pub fn source(&self) -> TriggerSource {
        self._tab
            .get::<TriggerSource>(Trigger::VT_SOURCE, Some(TriggerSource::TimestampReset))
            .unwrap()
    }
}

pub struct TriggerArgs {
    pub t: i64,
    pub source: TriggerSource,
}
impl<'a> Default for TriggerArgs {
    #[inline]
    fn default() -> Self {
        TriggerArgs {
            t: 0,
            source: TriggerSource::TimestampReset,
        }
    }
}
pub struct TriggerBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TriggerBuilder<'a, 'b> {
    #[inline]
    pub fn add_t(&mut self, t: i64) {
        self.fbb_.push_slot::<i64>(Trigger::VT_T, t, 0);
    }
    #[inline]
    pub fn add_source(&mut self, source: TriggerSource) {
        self.fbb_.push_slot::<TriggerSource>(
            Trigger::VT_SOURCE,
            source,
            TriggerSource::TimestampReset,
        );
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TriggerBuilder<'a, 'b> {
        let start = _fbb.start_table();
        TriggerBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Trigger<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

pub enum TriggerPacketOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct TriggerPacket<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TriggerPacket<'a> {
    type Inner = TriggerPacket<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> TriggerPacket<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TriggerPacket { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TriggerPacketArgs<'args>,
    ) -> flatbuffers::WIPOffset<TriggerPacket<'bldr>> {
        let mut builder = TriggerPacketBuilder::new(_fbb);
        if let Some(x) = args.elements {
            builder.add_elements(x);
        }
        builder.finish()
    }

    pub const VT_ELEMENTS: flatbuffers::VOffsetT = 4;

    #[inline]
    pub fn elements(
        &self,
    ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Trigger<'a>>>> {
        self._tab.get::<flatbuffers::ForwardsUOffset<
            flatbuffers::Vector<flatbuffers::ForwardsUOffset<Trigger<'a>>>,
        >>(TriggerPacket::VT_ELEMENTS, None)
    }
}

pub struct TriggerPacketArgs<'a> {
    pub elements: Option<
        flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Trigger<'a>>>>,
    >,
}
impl<'a> Default for TriggerPacketArgs<'a> {
    #[inline]
    fn default() -> Self {
        TriggerPacketArgs { elements: None }
    }
}
pub struct TriggerPacketBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TriggerPacketBuilder<'a, 'b> {
    #[inline]
    pub fn add_elements(
        &mut self,
        elements: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<Trigger<'b>>>,
        >,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(TriggerPacket::VT_ELEMENTS, elements);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TriggerPacketBuilder<'a, 'b> {
        let start = _fbb.start_table();
        TriggerPacketBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<TriggerPacket<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

#[inline]
pub fn get_root_as_trigger_packet<'a>(buf: &'a [u8]) -> TriggerPacket<'a> {
    flatbuffers::get_root::<TriggerPacket<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_trigger_packet<'a>(buf: &'a [u8]) -> TriggerPacket<'a> {
    flatbuffers::get_size_prefixed_root::<TriggerPacket<'a>>(buf)
}

pub const TRIGGER_PACKET_IDENTIFIER: &'static str = "TRIG";

#[inline]
pub fn trigger_packet_buffer_has_identifier(buf: &[u8]) -> bool {
    return flatbuffers::buffer_has_identifier(buf, TRIGGER_PACKET_IDENTIFIER, false);
}

#[inline]
pub fn trigger_packet_size_prefixed_buffer_has_identifier(buf: &[u8]) -> bool {
    return flatbuffers::buffer_has_identifier(buf, TRIGGER_PACKET_IDENTIFIER, true);
}

#[inline]
pub fn finish_trigger_packet_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<TriggerPacket<'a>>,
) {
    fbb.finish(root, Some(TRIGGER_PACKET_IDENTIFIER));
}

#[inline]
pub fn finish_size_prefixed_trigger_packet_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<TriggerPacket<'a>>,
) {
    fbb.finish_size_prefixed(root, Some(TRIGGER_PACKET_IDENTIFIER));
}
