use crate::format_context::FormatContext;
use crate::packet::Packet;

pub struct PacketIterator<'a> {
    format_context: &'a mut FormatContext
}

impl<'a> PacketIterator<'a> {
    pub fn new(format_context: &'a mut FormatContext) -> PacketIterator {
        PacketIterator { format_context }
    }
}

impl<'a> Iterator for PacketIterator<'a> {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        self.format_context.next_packet()
    }
}