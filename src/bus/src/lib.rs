/// Notes to myself:
///     - Implement some sort of subscribe mechanism that allow components to register their
///         ranges so we can iterate through the list to find out which of them read/write op is
///         meant to be carried out

pub struct Bus { }

impl Bus {
    pub fn new() -> Self {
        Bus{ }
    }

    pub fn read_address(addr: u16) -> u16 {
        //TODO implement logic to get data from the right component
        0
    }

    pub fn write_address(addr: u16) {
        //TODO implement logic to write data to the right component
    }
}