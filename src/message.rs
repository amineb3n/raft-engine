use crate::{Error, Result};
use raft_proto::protocompat::*;
use std::default::Default;

pub trait Message: PbMessage + Default {
    fn parse_from_bytes(bytes: &[u8]) -> Result<Self>;
    fn write_to_vec_compat(&self, vec: &mut Vec<u8>) -> Result<()>;
}

impl<T: PbMessage + Default> Message for T {
    #[inline]
    fn parse_from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut entry = T::default();
        entry.merge_from_bytes(bytes)?;
        Ok(entry)
    }

    #[inline]
    #[cfg(feature = "protobuf-codec")]
    fn write_to_vec_compat(&self, vec: &mut Vec<u8>) -> Result<()> {
        Ok(PbMessage::write_to_vec(self, vec)?)
    }

    #[inline]
    #[cfg(feature = "prost-codec")]
    fn write_to_vec_compat(&self, vec: &mut Vec<u8>) -> Result<()> {
        self.encode(vec)
            .map_err(|e| Error::ProstEncodeError(e.to_string()))?;
        Ok(())
    }
}
