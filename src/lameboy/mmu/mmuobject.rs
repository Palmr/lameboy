pub trait MmuObject {
    fn read8(&self, addr: u16) -> u8;
    fn write8(&mut self, addr: u16, data: u8);
}
