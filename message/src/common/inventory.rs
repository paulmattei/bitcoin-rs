use bitcrypto::SHA256D;
use ser::{Deserializable, Error as ReaderError, Reader, Serializable, Stream};
use std::io;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum InventoryType {
	Error = 0,
	MessageTx = 1,
	MessageBlock = 2,
	MessageFilteredBlock = 3,
	MessageCompactBlock = 4,
	MessageWitnessTx = 0x40000001,
	MessageWitnessBlock = 0x40000002,
	MessageWitnessFilteredBlock = 0x40000003,
}

impl InventoryType {
	pub fn from_u32(v: u32) -> Option<Self> {
		match v {
			0 => Some(InventoryType::Error),
			1 => Some(InventoryType::MessageTx),
			2 => Some(InventoryType::MessageBlock),
			3 => Some(InventoryType::MessageFilteredBlock),
			4 => Some(InventoryType::MessageCompactBlock),
			0x40000001 => Some(InventoryType::MessageWitnessTx),
			0x40000002 => Some(InventoryType::MessageWitnessBlock),
			0x40000003 => Some(InventoryType::MessageWitnessFilteredBlock),
			_ => None,
		}
	}
}

impl From<InventoryType> for u32 {
	fn from(i: InventoryType) -> Self {
		i as u32
	}
}

impl Serializable for InventoryType {
	fn serialize(&self, stream: &mut Stream) {
		stream.append(&u32::from(*self));
	}
}

impl Deserializable for InventoryType {
	fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, ReaderError>
	where
		T: io::Read,
	{
		let t: u32 = reader.read()?;
		InventoryType::from_u32(t).ok_or(ReaderError::MalformedData)
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct InventoryVector {
	pub inv_type: InventoryType,
	pub hash: SHA256D,
}

impl InventoryVector {
	pub fn tx(hash: SHA256D) -> Self {
		InventoryVector {
			inv_type: InventoryType::MessageTx,
			hash,
		}
	}

	pub fn witness_tx(hash: SHA256D) -> Self {
		InventoryVector {
			inv_type: InventoryType::MessageWitnessTx,
			hash,
		}
	}

	pub fn block(hash: SHA256D) -> Self {
		InventoryVector {
			inv_type: InventoryType::MessageBlock,
			hash,
		}
	}

	pub fn witness_block(hash: SHA256D) -> Self {
		InventoryVector {
			inv_type: InventoryType::MessageWitnessBlock,
			hash,
		}
	}
}

impl Serializable for InventoryVector {
	fn serialize(&self, stream: &mut Stream) {
		stream.append(&self.inv_type).append(&self.hash);
	}
}

impl Deserializable for InventoryVector {
	fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, ReaderError>
	where
		T: io::Read,
	{
		let vec = InventoryVector {
			inv_type: reader.read()?,
			hash: reader.read()?,
		};

		Ok(vec)
	}
}

#[cfg(test)]
mod tests {
	use super::SHA256D;
	use super::{InventoryType, InventoryVector};
	use crate::bytes::Bytes;
	use bitcrypto::FromStr;
	use ser::{deserialize, serialize};

	#[test]
	fn test_inventory_serialize() {
		let expected = "020000000400000000000000000000000000000000000000000000000000000000000000".into();

		let inventory = InventoryVector {
			inv_type: InventoryType::MessageBlock,
			hash: SHA256D::from_str("0000000000000000000000000000000000000000000000000000000000000004").unwrap(),
		};

		assert_eq!(serialize(&inventory), expected);
	}

	#[test]
	fn test_inventory_deserialize() {
		let raw: Bytes = "020000000400000000000000000000000000000000000000000000000000000000000000".into();

		let expected = InventoryVector {
			inv_type: InventoryType::MessageBlock,
			hash: SHA256D::from_str("0000000000000000000000000000000000000000000000000000000000000004").unwrap(),
		};

		assert_eq!(expected, deserialize(raw.as_ref()).unwrap());
	}

	#[test]
	fn test_inventory_type_conversion() {
		assert_eq!(0u32, InventoryType::Error.into());
		assert_eq!(1u32, InventoryType::MessageTx.into());
		assert_eq!(2u32, InventoryType::MessageBlock.into());
		assert_eq!(3u32, InventoryType::MessageFilteredBlock.into());
		assert_eq!(4u32, InventoryType::MessageCompactBlock.into());
		assert_eq!(0x40000001u32, InventoryType::MessageWitnessTx.into());
		assert_eq!(0x40000002u32, InventoryType::MessageWitnessBlock.into());
		assert_eq!(0x40000003u32, InventoryType::MessageWitnessFilteredBlock.into());

		assert_eq!(InventoryType::from_u32(0).unwrap(), InventoryType::Error);
		assert_eq!(InventoryType::from_u32(1).unwrap(), InventoryType::MessageTx);
		assert_eq!(InventoryType::from_u32(2).unwrap(), InventoryType::MessageBlock);
		assert_eq!(InventoryType::from_u32(3).unwrap(), InventoryType::MessageFilteredBlock);
		assert_eq!(InventoryType::from_u32(4).unwrap(), InventoryType::MessageCompactBlock);
		assert_eq!(InventoryType::from_u32(0x40000001).unwrap(), InventoryType::MessageWitnessTx);
		assert_eq!(InventoryType::from_u32(0x40000002).unwrap(), InventoryType::MessageWitnessBlock);
		assert_eq!(
			InventoryType::from_u32(0x40000003).unwrap(),
			InventoryType::MessageWitnessFilteredBlock
		);
	}
}
