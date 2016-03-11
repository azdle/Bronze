#[derive(PartialEq, Eq, Debug)]
pub struct Message {
    pub version: u8,
    pub mtype: Mtype,
    pub code: Code,
    pub mid: u16,
    pub token: Vec<u8>,
    pub options: Vec<option::Option>,
    pub payload: Vec<u8>,
}

#[derive(PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Mtype {
    Confirmable = 0,
    NonConfirmable = 1,
    Acknowledgement = 2,
    Reset = 3,
}

impl Into<u8> for Mtype {
    fn into(self) -> u8 {
        self as u8
    }
}

impl PartialEq<u8> for Mtype {
    fn eq(&self, other: &u8) -> bool {
        *self as u8 == *other
    }

    fn ne(&self, other: &u8) -> bool {
        *self as u8  != *other
    }
}

macro_rules! code {
    ($c:expr, $d:expr) => ((($c & 0x07) << 5) | ($d & 0x1F))
}

#[derive(PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Code {
    Empty = code!(0,00),
    Get = code!(0,01),
    Post = code!(0,02),
    Put = code!(0,03),
    Delete = code!(0,04),
    Created = code!(2,01),
    Deleted = code!(2,02),
    Valid = code!(2,03),
    Changed = code!(2,04),
    Content = code!(2,05),
    BadRequest = code!(4,00),
    Unauthorized = code!(4,01),
    BadOption = code!(4,02),
    Forbidden = code!(4,03),
    NotFound = code!(4,04),
    MethodNotAllowed = code!(4,05),
    NotAcceptable = code!(4,06),
    PreconditionFailed = code!(4,12),
    RequestEntityTooLarge = code!(4,13),
    UnsupportedContentFormat = code!(4,15),
    InternalServerError = code!(5,00),
    NotImplemented = code!(5,01),
    BadGateway = code!(5,02),
    ServiceUnavailable = code!(5,03),
    GatewayTimeout = code!(5,04),
    ProxyingNotSupported = code!(5,05),
}

pub mod option {
    #[derive(PartialEq, Eq, Debug)]
    pub struct Option {
        pub number: Number,
        pub value: Vec<u8>
    }
    #[derive(PartialEq, Eq, Debug)]
    #[repr(u16)]
    pub enum Number {
        ReservedOrUnknown = 0,
        IfMatch = 1,
        UriHost = 3,
        ETag = 4,
        IfNoneMatch = 5,
        Observe = 6,
        UriPort = 7,
        LocationPath = 8,
        UriPath = 11,
        ContentFormat = 12,
        MaxAge = 14,
        UriQuery = 15,
        Accept = 17,
        LocationQuery = 20,
        ProxyUri = 35,
        ProxyScheme = 39,
        Size1 = 60,
        NoResponse = 284,
    }

    pub fn new(number: Number, value: Vec<u8>) -> Option {
        Option{
            number: number,
            value: value
        }
    }

    pub fn from_u16(numeric: u16, value: Vec<u8>) -> Option {
        use std::mem;

        new(unsafe { mem::transmute(numeric)}, value)
    }
}


impl Message {
    pub fn from_bin(pkt: &[u8]) -> Result<Message, &str> {
        use std::mem;

        let mut i: usize;

        let version = pkt[0] >> 6;
        let mtype = unsafe{ mem::transmute((pkt[0] >> 4) & 0x03) };
        let token_length = pkt[0] & 0x0F;
        let code = unsafe{ mem::transmute(pkt[1]) };
        let mid = ((pkt[2] as u16) << 8) | pkt[3] as u16;

        let mut token = vec![];
        for j in 0..token_length {
            token.push(pkt[4+j as usize]);
        }

        i = 4 + token_length as usize;

        let mut options: Vec<option::Option> = vec![];
        let mut option_number_offset = 0u16;

        while i < pkt.len() {
            if pkt[i] == 0xFF {
                i+=1;
                break;
            }

            // Note: length errors for 13 & 14 will be caught in the check below.
            let delta = match pkt[i] >> 4 {
                d @ 0 ... 12 => d as u16,
                13 => {i+=1; pkt[i] as u16 + 13},
                14 => {i+=2; (((pkt[i-1] as u16) << 8) | pkt[i] as u16) + 269},
                15 => panic!("message format error"),
                _  => unreachable!(),
            };
            let length = match pkt[i] & 0x0F {
                d @ 0 ... 12 => d as u16,
                13 => {i+=1; pkt[i] as u16 + 13},
                14 => {i+=2; ((pkt[i-1] as u16) << 8) | pkt[i] as u16 + 269},
                15 => panic!("message format error"),
                _  => unreachable!(),
            };

            i += 1;

            let option_number = option_number_offset + delta;
            option_number_offset = option_number;

            if length >= 65000 {
                return Err("message format error");
            }

            if pkt.len() >= i+(length as usize) {
                options.push(option::from_u16(option_number,pkt[i..i+(length as usize)].to_vec()));
            } else {
                return Err("message format error");
            }

            i += length as usize;
        }

        let payload = if i < pkt.len() {
            pkt[i..].to_vec()
        } else {
            vec![]
        };

        Ok(Message{
            version: version,
            mtype: mtype,
            code: code,
            mid: mid,
            token: token,
            options: options,
            payload: payload,
        })
    }
}



#[test]
fn test_msg_parse_empty() {
    let ref_bin: [u8;4] = [64,0,0,0];

    let msg = Message::from_bin(&ref_bin).unwrap();

    assert!(msg.version == 1);
    assert!(msg.mtype == Mtype::Confirmable);
    assert!(msg.code == Code::Empty);
    assert!(msg.mid == 0);
    assert!(msg.token == vec![]);
    assert!(msg.options == vec![]);
    assert!(msg.payload == vec![]);
}

#[test]
fn test_msg_parse_empty_con_with_token() {
    let ref_bin: [u8;6] = [66,0,0,0,37,42];

    let msg = Message::from_bin(&ref_bin).unwrap();

    assert!(msg.version == 1);
    assert!(msg.mtype == Mtype::Confirmable);
    assert!(msg.code == Code::Empty);
    assert!(msg.mid == 0);
    assert!(msg.token == vec![37, 42]);
    assert!(msg.options == vec![]);
    assert!(msg.payload == vec![]);
}

#[test]
fn test_msg_parse_get_con() {
    let ref_bin: [u8;8] = [0x41,0x01,0x00,0x37,0x99,0xFF,0x01,0x02];

    let msg = Message::from_bin(&ref_bin).unwrap();

    assert!(msg.version == 1);
    assert!(msg.mtype == Mtype::Confirmable);
    assert!(msg.code == Code::Get);
    assert!(msg.mid == 0x37);
    assert!(msg.token == vec![0x99]);
    assert!(msg.options == vec![]);
    assert!(msg.payload == vec![0x01, 0x02]);
}

#[test]
fn test_msg_parse_get_con_with_opts() {
    let ref_bin: [u8;57] = [0x40,0x02,0x00,0x37,0xb2,0x31,0x61,0x04,0x74,0x65,
	                       0x6d,0x70,0x4d,0x1b,0x61,0x33,0x32,0x63,0x38,0x35,
	                       0x62,0x61,0x39,0x64,0x64,0x61,0x34,0x35,0x38,0x32,
	                       0x33,0x62,0x65,0x34,0x31,0x36,0x32,0x34,0x36,0x63,
	                       0x66,0x38,0x62,0x34,0x33,0x33,0x62,0x61,0x61,0x30,
	                       0x36,0x38,0x64,0x37,0xFF,0x39,0x39];

    let msg = Message::from_bin(&ref_bin).unwrap();

    assert!(msg.version == 1);
    assert!(msg.mtype == Mtype::Confirmable);
    assert!(msg.code == Code::Post);
    assert!(msg.mid == 0x0037);
    assert!(msg.token == vec![]);
    assert!(msg.options == vec![
        option::Option{
            number: option::Number::UriPath,
            value: vec![0x31,0x61]
        },
        option::Option{
            number: option::Number::UriPath,
            value: vec![0x74,0x65,0x6d,0x70]
        },
        option::Option{
            number: option::Number::UriQuery,
            value: vec![0x61,0x33,0x32,0x63,0x38,0x35,0x62,0x61,0x39,0x64,
                        0x64,0x61,0x34,0x35,0x38,0x32,0x33,0x62,0x65,0x34,
                        0x31,0x36,0x32,0x34,0x36,0x63,0x66,0x38,0x62,0x34,
                        0x33,0x33,0x62,0x61,0x61,0x30,0x36,0x38,0x64,0x37]
        },
    ]);
    assert!(msg.payload == vec![0x39, 0x39]);
}
