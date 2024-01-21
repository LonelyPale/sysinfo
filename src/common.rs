pub trait PrettySize {
    fn pretty_size(self) -> String;
}

impl PrettySize for u64 {
    fn pretty_size(self) -> String {
        let n = self as f64;
        if n < K {
            format!("{}B", self)
        } else if n < M {
            format!("{:.2}K", n / K)
        } else if n < G {
            format!("{:.2}M", n / M)
        } else if n < T {
            format!("{:.2}G", n / G)
        } else if n < P {
            format!("{:.2}T", n / T)
        } else if n < E {
            format!("{:.2}P", n / P)
        } else if n < Z {
            format!("{:.2}E", n / E)
        } else if n < Y {
            format!("{:.2}Z", n / Z)
        } else if n < BB {
            format!("{:.2}Y", n / Y)
        } else {
            format!("{:.2}BB", n / BB)
        }
    }
}

const B: f64 = 1.0; //1字节(Byte)=8位(bit)

const K: f64 = 1024.0 * B; //1KB(KiloByte，千字节)=1024B

const M: f64 = 1024.0 * K; //1MB(MegaByte，兆字节)=1024KB

const G: f64 = 1024.0 * M; //1GB(GigaByte，吉字节，千兆)=1024MB

const T: f64 = 1024.0 * G; //1TB(TrillionByte，万亿字节，太字节)=1024GB

const P: f64 = 1024.0 * T; //1PB(PetaByte，千万亿字节，拍字节)=1024TB

const E: f64 = 1024.0 * P; //1EB(ExaByte，百亿亿字节，艾字节)=1024PB

// error: attempt to compute `1024_u64 * 1152921504606846976_u64`, which would overflow E以后的用u64都会溢出，f64则不会

const Z: f64 = 1024.0 * E; //1ZB(ZettaByte，十万亿亿字节，泽字节)=1024EB

const Y: f64 = 1024.0 * Z; //1YB(YottaByte，一亿亿亿字节，尧字节)=1024ZB

const BB: f64 = 1024.0 * Y; //1BB(BrontoByte，千亿亿亿字节)=1024YB
