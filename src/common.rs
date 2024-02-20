use std::fmt;
use std::error;
use std::str::FromStr;

/// 1B(Byte，字节)=8位(bit)
/// 1KB(KiloByte，千字节)=1024B
/// 1MB(MegaByte，兆字节)=1024KB
/// 1GB(GigaByte，吉字节，千兆)=1024MB
/// 1TB(TrillionByte，万亿字节，太字节)=1024GB
/// 1PB(PetaByte，千万亿字节，拍字节)=1024TB
/// 1EB(ExaByte，百亿亿字节，艾字节)=1024PB
/// 1ZB(ZettaByte，十万亿亿字节，泽字节)=1024EB
/// 1YB(YottaByte，一亿亿亿字节，尧字节)=1024ZB
/// 1BB(BrontoByte，千亿亿亿字节)=1024YB
// error: attempt to compute `1024_u64 * 1152921504606846976_u64`, which would overflow EB以后的用u64都会溢出，f64则不会

//
#[derive(Debug, Copy, Clone)]
pub enum BaseSize {
    Size1024 = 1024,
    Size1000 = 1000,
}

#[derive(Debug, Copy, Clone)]
pub enum BlockSize {
    Auto,
    B,
    K,
    M,
    G,
    T,
    P,
    E,
    Z,
    Y,
    BB,
}

#[derive(Debug)]
pub struct BlockSizeParseError;

impl fmt::Display for BlockSizeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid BlockSize")
    }
}

impl error::Error for BlockSizeParseError {}

impl FromStr for BlockSize {
    type Err = BlockSizeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binding = s.to_uppercase();
        let block_size = match binding.as_str() {
            "AUTO" => { BlockSize::Auto }
            "B" => { BlockSize::B }
            "K" => { BlockSize::K }
            "M" => { BlockSize::M }
            "G" => { BlockSize::G }
            "T" => { BlockSize::T }
            "P" => { BlockSize::P }
            "E" => { BlockSize::E }
            "Z" => { BlockSize::Z }
            "Y" => { BlockSize::Y }
            "BB" => { BlockSize::BB }
            _ => { return Err(BlockSizeParseError); }
        };
        Ok(block_size)
    }
}

pub struct Size {
    pub k: f64,
    pub m: f64,
    pub g: f64,
    pub t: f64,
    pub p: f64,
    pub e: f64,
    pub z: f64,
    pub y: f64,
    pub bb: f64,
}

const SIZE_1024: Size = Size {
    k: 1024.0,
    m: 1024.0 * 1024.0,
    g: 1024.0 * 1024.0 * 1024.0,
    t: 1024.0 * 1024.0 * 1024.0 * 1024.0,
    p: 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
    e: 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
    z: 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
    y: 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
    bb: 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
};

const SIZE_1000: Size = Size {
    k: 1000.0,
    m: 1000.0 * 1000.0,
    g: 1000.0 * 1000.0 * 1000.0,
    t: 1000.0 * 1000.0 * 1000.0 * 1000.0,
    p: 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0,
    e: 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0,
    z: 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0,
    y: 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0,
    bb: 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0,
};

pub trait PrettySize {
    fn pretty_size(self) -> String;
    fn pretty_size_with(self, base: BaseSize, block: BlockSize) -> String;
}

impl PrettySize for u64 {
    fn pretty_size(self) -> String {
        self.pretty_size_with(BaseSize::Size1024, BlockSize::Auto)
    }

    fn pretty_size_with(self, base: BaseSize, block: BlockSize) -> String {
        if let BlockSize::Auto = block {
            format_auto(self, base)
        } else {
            let n = self as f64;
            let size: Size = match base {
                BaseSize::Size1024 => { SIZE_1024 }
                BaseSize::Size1000 => { SIZE_1000 }
            };
            match block {
                BlockSize::B => { format!("{}B", n) }
                BlockSize::K => { format!("{:.2}K", n / size.k) }
                BlockSize::M => { format!("{:.2}M", n / size.m) }
                BlockSize::G => { format!("{:.2}G", n / size.g) }
                BlockSize::T => { format!("{:.2}T", n / size.t) }
                BlockSize::P => { format!("{:.2}P", n / size.p) }
                BlockSize::E => { format!("{:.2}E", n / size.e) }
                BlockSize::Z => { format!("{:.2}Z", n / size.z) }
                BlockSize::Y => { format!("{:.2}Y", n / size.y) }
                BlockSize::BB => { format!("{:.2}BB", n / size.bb) }
                _ => "".to_string()
            }
        }
    }
}

fn format_auto(num: u64, base: BaseSize) -> String {
    let n = num as f64;
    let size: Size = match base {
        BaseSize::Size1024 => { SIZE_1024 }
        BaseSize::Size1000 => { SIZE_1000 }
    };

    if n < size.k {
        format!("{}B", n)
    } else if n < size.m {
        format!("{:.2}K", n / size.k)
    } else if n < size.g {
        format!("{:.2}M", n / size.m)
    } else if n < size.t {
        format!("{:.2}G", n / size.g)
    } else if n < size.p {
        format!("{:.2}T", n / size.t)
    } else if n < size.e {
        format!("{:.2}P", n / size.p)
    } else if n < size.z {
        format!("{:.2}E", n / size.e)
    } else if n < size.y {
        format!("{:.2}Z", n / size.z)
    } else if n < size.bb {
        format!("{:.2}Y", n / size.y)
    } else {
        format!("{:.2}BB", n / size.bb)
    }
}

#[test]
fn test() {
    let block_size: u64 = 4096;
    let free_blocks: u64 = 5429129;
    let free = free_blocks * block_size;
    println!("{}", free.pretty_size());

    let other_a = 1 << 10;
    println!("2^10={}", other_a);
}

#[test]
fn test_block_size_parse() {
    let block_size: BlockSize = "B".parse().unwrap();
    println!("{:?}", block_size);

    let result: Result<BlockSize, _> = "".parse();
    match result {
        Ok(val) => { println!("{:?}", val) }
        Err(err) => { println!("{}", err) }
    }

    // let _block_size: BlockSize = "".parse().expect("Failed to parse the BlockSize");
}

// Block size: 4096 bytes
// Total blocks: 491968500
// Free blocks: 5429129
// Available blocks: 0
// Inodes: 125026304
// Free inodes: 125026192
// Name max length: 255
