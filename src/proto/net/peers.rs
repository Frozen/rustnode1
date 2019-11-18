use crate::errors::ConvertError;
use crate::proto::deserializer::Deserializer;
use crate::proto::Serializer;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

#[derive(Debug, Eq, PartialEq)]
pub struct PeersMessage(pub Vec<SocketAddr>);

impl PeersMessage {
    pub fn deserialize<R: io::Read>(r: &mut R) -> Result<PeersMessage, ConvertError> {
        let size = r.read_u32::<BigEndian>()?;
        let mut out: Vec<SocketAddr> = vec![];
        for i in 0..size {
            let s = SocketAddrV4::new(
                Ipv4Addr::new(r.byte()?, r.byte()?, r.byte()?, r.byte()?),
                r.read_u32::<BigEndian>()? as u16,
            );
            out.push(SocketAddr::V4(s));
        }
        Ok(PeersMessage(out))
    }

    pub fn serialize(&self, s: &mut Serializer) -> Result<(), ConvertError> {
        s.u32(self.0.len() as u32)?;

        for i in 0..self.0.len() {
            match self.0[i].ip() {
                IpAddr::V4(ref addr) => {
                    let o = addr.octets();
                    s.byte(o[0]);
                    s.byte(o[1]);
                    s.byte(o[2]);
                    s.byte(o[3]);
                    s.u32(self.0[i].port() as u32)?;
                }
                IpAddr::V6(..) => unimplemented!(),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::net::SocketAddrV4;

    #[test]
    fn serde() {
        let vec: Vec<SocketAddr> = ["127.0.0.1:6868", "178.62.225.183:80"]
            .iter()
            .flat_map(|x| x.parse())
            .collect();

        let v = PeersMessage(vec);

        let mut s = Serializer::new();

        v.serialize(&mut s).unwrap();

        let mut cursor = Cursor::new(s.as_bytes());

        let v2 = PeersMessage::deserialize(&mut cursor).unwrap();

        assert_eq!(v, v2)
    }
}
