use crate::format_context::FormatContext;
use crate::packet::Packet;

pub struct PacketIterator<'a> {
    format_context: &'a mut FormatContext,
    last_packet: Option<Packet>
}

impl<'a> PacketIterator<'a> {
    pub fn new(format_context: &'a mut FormatContext) -> PacketIterator {
        PacketIterator { format_context, last_packet: Some(Packet::new(true)) }
    }
}

impl<'a> Iterator for PacketIterator<'a> {
    type Item = &'a mut Packet;

    fn next(&mut self) -> Option<Self::Item> {
        self.last_packet = self.format_context.next_packet(self.last_packet.take()?);

        let packet = self.last_packet.as_mut()?;
        unsafe { Some(&mut *(packet as *mut Packet)) }
    }
}