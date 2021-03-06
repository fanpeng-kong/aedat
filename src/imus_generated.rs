// automatically generated by the FlatBuffers compiler, do not modify

use std::cmp::Ordering;
use std::mem;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

pub enum ImuOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Imu<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Imu<'a> {
    type Inner = Imu<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Imu<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Imu { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ImuArgs,
    ) -> flatbuffers::WIPOffset<Imu<'bldr>> {
        let mut builder = ImuBuilder::new(_fbb);
        builder.add_t(args.t);
        builder.add_magnetometer_z(args.magnetometer_z);
        builder.add_magnetometer_y(args.magnetometer_y);
        builder.add_magnetometer_x(args.magnetometer_x);
        builder.add_gyroscope_z(args.gyroscope_z);
        builder.add_gyroscope_y(args.gyroscope_y);
        builder.add_gyroscope_x(args.gyroscope_x);
        builder.add_accelerometer_z(args.accelerometer_z);
        builder.add_accelerometer_y(args.accelerometer_y);
        builder.add_accelerometer_x(args.accelerometer_x);
        builder.add_temperature(args.temperature);
        builder.finish()
    }

    pub const VT_T: flatbuffers::VOffsetT = 4;
    pub const VT_TEMPERATURE: flatbuffers::VOffsetT = 6;
    pub const VT_ACCELEROMETER_X: flatbuffers::VOffsetT = 8;
    pub const VT_ACCELEROMETER_Y: flatbuffers::VOffsetT = 10;
    pub const VT_ACCELEROMETER_Z: flatbuffers::VOffsetT = 12;
    pub const VT_GYROSCOPE_X: flatbuffers::VOffsetT = 14;
    pub const VT_GYROSCOPE_Y: flatbuffers::VOffsetT = 16;
    pub const VT_GYROSCOPE_Z: flatbuffers::VOffsetT = 18;
    pub const VT_MAGNETOMETER_X: flatbuffers::VOffsetT = 20;
    pub const VT_MAGNETOMETER_Y: flatbuffers::VOffsetT = 22;
    pub const VT_MAGNETOMETER_Z: flatbuffers::VOffsetT = 24;

    #[inline]
    pub fn t(&self) -> i64 {
        self._tab.get::<i64>(Imu::VT_T, Some(0)).unwrap()
    }
    #[inline]
    pub fn temperature(&self) -> f32 {
        self._tab
            .get::<f32>(Imu::VT_TEMPERATURE, Some(0.0))
            .unwrap()
    }
    #[inline]
    pub fn accelerometer_x(&self) -> f32 {
        self._tab
            .get::<f32>(Imu::VT_ACCELEROMETER_X, Some(0.0))
            .unwrap()
    }
    #[inline]
    pub fn accelerometer_y(&self) -> f32 {
        self._tab
            .get::<f32>(Imu::VT_ACCELEROMETER_Y, Some(0.0))
            .unwrap()
    }
    #[inline]
    pub fn accelerometer_z(&self) -> f32 {
        self._tab
            .get::<f32>(Imu::VT_ACCELEROMETER_Z, Some(0.0))
            .unwrap()
    }
    #[inline]
    pub fn gyroscope_x(&self) -> f32 {
        self._tab
            .get::<f32>(Imu::VT_GYROSCOPE_X, Some(0.0))
            .unwrap()
    }
    #[inline]
    pub fn gyroscope_y(&self) -> f32 {
        self._tab
            .get::<f32>(Imu::VT_GYROSCOPE_Y, Some(0.0))
            .unwrap()
    }
    #[inline]
    pub fn gyroscope_z(&self) -> f32 {
        self._tab
            .get::<f32>(Imu::VT_GYROSCOPE_Z, Some(0.0))
            .unwrap()
    }
    #[inline]
    pub fn magnetometer_x(&self) -> f32 {
        self._tab
            .get::<f32>(Imu::VT_MAGNETOMETER_X, Some(0.0))
            .unwrap()
    }
    #[inline]
    pub fn magnetometer_y(&self) -> f32 {
        self._tab
            .get::<f32>(Imu::VT_MAGNETOMETER_Y, Some(0.0))
            .unwrap()
    }
    #[inline]
    pub fn magnetometer_z(&self) -> f32 {
        self._tab
            .get::<f32>(Imu::VT_MAGNETOMETER_Z, Some(0.0))
            .unwrap()
    }
}

pub struct ImuArgs {
    pub t: i64,
    pub temperature: f32,
    pub accelerometer_x: f32,
    pub accelerometer_y: f32,
    pub accelerometer_z: f32,
    pub gyroscope_x: f32,
    pub gyroscope_y: f32,
    pub gyroscope_z: f32,
    pub magnetometer_x: f32,
    pub magnetometer_y: f32,
    pub magnetometer_z: f32,
}
impl<'a> Default for ImuArgs {
    #[inline]
    fn default() -> Self {
        ImuArgs {
            t: 0,
            temperature: 0.0,
            accelerometer_x: 0.0,
            accelerometer_y: 0.0,
            accelerometer_z: 0.0,
            gyroscope_x: 0.0,
            gyroscope_y: 0.0,
            gyroscope_z: 0.0,
            magnetometer_x: 0.0,
            magnetometer_y: 0.0,
            magnetometer_z: 0.0,
        }
    }
}
pub struct ImuBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ImuBuilder<'a, 'b> {
    #[inline]
    pub fn add_t(&mut self, t: i64) {
        self.fbb_.push_slot::<i64>(Imu::VT_T, t, 0);
    }
    #[inline]
    pub fn add_temperature(&mut self, temperature: f32) {
        self.fbb_
            .push_slot::<f32>(Imu::VT_TEMPERATURE, temperature, 0.0);
    }
    #[inline]
    pub fn add_accelerometer_x(&mut self, accelerometer_x: f32) {
        self.fbb_
            .push_slot::<f32>(Imu::VT_ACCELEROMETER_X, accelerometer_x, 0.0);
    }
    #[inline]
    pub fn add_accelerometer_y(&mut self, accelerometer_y: f32) {
        self.fbb_
            .push_slot::<f32>(Imu::VT_ACCELEROMETER_Y, accelerometer_y, 0.0);
    }
    #[inline]
    pub fn add_accelerometer_z(&mut self, accelerometer_z: f32) {
        self.fbb_
            .push_slot::<f32>(Imu::VT_ACCELEROMETER_Z, accelerometer_z, 0.0);
    }
    #[inline]
    pub fn add_gyroscope_x(&mut self, gyroscope_x: f32) {
        self.fbb_
            .push_slot::<f32>(Imu::VT_GYROSCOPE_X, gyroscope_x, 0.0);
    }
    #[inline]
    pub fn add_gyroscope_y(&mut self, gyroscope_y: f32) {
        self.fbb_
            .push_slot::<f32>(Imu::VT_GYROSCOPE_Y, gyroscope_y, 0.0);
    }
    #[inline]
    pub fn add_gyroscope_z(&mut self, gyroscope_z: f32) {
        self.fbb_
            .push_slot::<f32>(Imu::VT_GYROSCOPE_Z, gyroscope_z, 0.0);
    }
    #[inline]
    pub fn add_magnetometer_x(&mut self, magnetometer_x: f32) {
        self.fbb_
            .push_slot::<f32>(Imu::VT_MAGNETOMETER_X, magnetometer_x, 0.0);
    }
    #[inline]
    pub fn add_magnetometer_y(&mut self, magnetometer_y: f32) {
        self.fbb_
            .push_slot::<f32>(Imu::VT_MAGNETOMETER_Y, magnetometer_y, 0.0);
    }
    #[inline]
    pub fn add_magnetometer_z(&mut self, magnetometer_z: f32) {
        self.fbb_
            .push_slot::<f32>(Imu::VT_MAGNETOMETER_Z, magnetometer_z, 0.0);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ImuBuilder<'a, 'b> {
        let start = _fbb.start_table();
        ImuBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Imu<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

pub enum ImuPacketOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct ImuPacket<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ImuPacket<'a> {
    type Inner = ImuPacket<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> ImuPacket<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        ImuPacket { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ImuPacketArgs<'args>,
    ) -> flatbuffers::WIPOffset<ImuPacket<'bldr>> {
        let mut builder = ImuPacketBuilder::new(_fbb);
        if let Some(x) = args.elements {
            builder.add_elements(x);
        }
        builder.finish()
    }

    pub const VT_ELEMENTS: flatbuffers::VOffsetT = 4;

    #[inline]
    pub fn elements(
        &self,
    ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Imu<'a>>>> {
        self._tab.get::<flatbuffers::ForwardsUOffset<
            flatbuffers::Vector<flatbuffers::ForwardsUOffset<Imu<'a>>>,
        >>(ImuPacket::VT_ELEMENTS, None)
    }
}

pub struct ImuPacketArgs<'a> {
    pub elements: Option<
        flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Imu<'a>>>>,
    >,
}
impl<'a> Default for ImuPacketArgs<'a> {
    #[inline]
    fn default() -> Self {
        ImuPacketArgs { elements: None }
    }
}
pub struct ImuPacketBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ImuPacketBuilder<'a, 'b> {
    #[inline]
    pub fn add_elements(
        &mut self,
        elements: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<Imu<'b>>>,
        >,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(ImuPacket::VT_ELEMENTS, elements);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ImuPacketBuilder<'a, 'b> {
        let start = _fbb.start_table();
        ImuPacketBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<ImuPacket<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

#[inline]
pub fn get_root_as_imu_packet<'a>(buf: &'a [u8]) -> ImuPacket<'a> {
    flatbuffers::get_root::<ImuPacket<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_imu_packet<'a>(buf: &'a [u8]) -> ImuPacket<'a> {
    flatbuffers::get_size_prefixed_root::<ImuPacket<'a>>(buf)
}

pub const IMU_PACKET_IDENTIFIER: &'static str = "IMUS";

#[inline]
pub fn imu_packet_buffer_has_identifier(buf: &[u8]) -> bool {
    return flatbuffers::buffer_has_identifier(buf, IMU_PACKET_IDENTIFIER, false);
}

#[inline]
pub fn imu_packet_size_prefixed_buffer_has_identifier(buf: &[u8]) -> bool {
    return flatbuffers::buffer_has_identifier(buf, IMU_PACKET_IDENTIFIER, true);
}

#[inline]
pub fn finish_imu_packet_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<ImuPacket<'a>>,
) {
    fbb.finish(root, Some(IMU_PACKET_IDENTIFIER));
}

#[inline]
pub fn finish_size_prefixed_imu_packet_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<ImuPacket<'a>>,
) {
    fbb.finish_size_prefixed(root, Some(IMU_PACKET_IDENTIFIER));
}
