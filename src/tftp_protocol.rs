
pub mod tftp {

    pub const BLOCK_SIZE : usize = 512;

    #[repr(u16)]
    pub enum OpCode {
        RRQ = 1,
        WRQ = 2,
        DATA = 3,
        ACK = 4,
        ERROR = 5
    }

    //  2 bytes     string    1 byte     string   1 byte
    // --------------------------------------------------
    // | Opcode |  Filename  |   0  |    Mode    |   0  |
    // --------------------------------------------------
    pub struct ReadRequest {
        pub filename: String,
        pub mode: String
    }

    //  2 bytes     2 bytes      n bytes
    // ------------------------------------
    // | Opcode |   Block #  |   Data     |
    // ------------------------------------
    pub struct Data {
        pub block_number: u16,
        pub data: Vec<u8>
    }

    //  2 bytes     2 bytes
    // -----------------------
    // | Opcode |   Block #  |
    // -----------------------
    pub struct Acknowledge {
        pub block_number: u16
    }

    //  2 bytes     2 bytes      string    1 byte
    // -------------------------------------------
    // | Opcode |  ErrorCode |   ErrMsg   |   0  |
    // -------------------------------------------
    pub struct Error {
        pub error_code: u16,
        pub error_message: String
    }

}

