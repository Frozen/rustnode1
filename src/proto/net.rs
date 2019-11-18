use crate::errors::ConvertError;
use crate::proto::deserializer::Deserializer;
use crate::proto::Serializer;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

// version represents the version of the protocol
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Handshake {
    app_name: String,
    version: Version,
    node_name: String,
    node_nonce: u64,
    declared_addr: Option<SocketAddr>,
    timestamp: u64,
}

impl Handshake {
    pub fn deserialize<R: io::Read>(r: &mut R) -> Result<Handshake, ConvertError> {
        let appname = r.u8string()?;

        let version = Version {
            major: r.read_u32::<BigEndian>()?,
            minor: r.read_u32::<BigEndian>()?,
            patch: r.read_u32::<BigEndian>()?,
        };

        let node_name = r.u8string()?;
        let nonce = r.u64()?;

        let decl_addr = parse_addr(r)?;
        let timestamp = r.timestamp()?;

        Ok(Handshake {
            app_name: appname,
            version,
            node_name,
            node_nonce: nonce,
            declared_addr: decl_addr,
            timestamp,
        })
    }

    pub fn serialize(&self, s: &mut Serializer) -> Result<(), ConvertError> {
        s.u8string(&self.app_name)?;
        s.u32(self.version.major)?;
        s.u32(self.version.minor)?;
        s.u32(self.version.patch)?;
        s.u8string(&self.node_name)?;
        s.u64(self.node_nonce)?;

        match self.declared_addr {
            Some(ref t) => {
                s.u32(8)?;
                match t.ip() {
                    IpAddr::V4(ref v4) => {
                        let o = v4.octets();
                        s.byte(o[0]);
                        s.byte(o[1]);
                        s.byte(o[2]);
                        s.byte(o[3]);
                        s.u32(t.port() as u32)?
                    }
                    IpAddr::V6(_) => unreachable!(),
                }
            }
            None => s.u32(0)?,
        };
        s.u64(self.timestamp)?;
        Ok(())
    }
}

fn parse_addr<R: io::Read>(r: &mut R) -> Result<Option<SocketAddr>, ConvertError> {
    let size = r.read_u32::<BigEndian>()?;
    match size {
        8 => Ok(Some(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(r.byte()?, r.byte()?, r.byte()?, r.byte()?)),
            r.read_u32::<BigEndian>()? as u16,
        ))),
        0 => Ok(None),
        _ => bad_args!("can't parse tcp addr; expected size 0 or 8, found {}", size),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::net::SocketAddrV4;

    #[test]
    fn serde() {
        let h = Handshake {
            app_name: "wavesW".to_string(),
            version: Version {
                major: 1,
                minor: 1,
                patch: 5,
            },
            node_name: "rustwaves".to_string(),
            node_nonce: 100500,
            declared_addr: "78.46.193.104:6868".parse().map(|x| Some(x)).unwrap(),
            timestamp: 1999777,
        };

        let mut s = Serializer::new();
        h.serialize(&mut s).unwrap();
        let mut cur = Cursor::new(s.as_bytes());
        let h2 = Handshake::deserialize(&mut cur).unwrap();
        assert_eq!(h, h2);
    }
}
