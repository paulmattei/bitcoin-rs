use crate::common::InventoryVector;
use crate::{MessageResult, Payload};
use ser::{Reader, Stream};
use std::io;

pub const INV_MAX_INVENTORY_LEN: usize = 50_000;

#[derive(Debug, PartialEq)]
pub struct Inv {
	pub inventory: Vec<InventoryVector>,
}

impl Inv {
	pub fn with_inventory(inventory: Vec<InventoryVector>) -> Self {
		Inv { inventory }
	}
}

impl Payload for Inv {
	fn version() -> u32 {
		0
	}

	fn command() -> &'static str {
		"inv"
	}

	fn deserialize_payload<T>(reader: &mut Reader<T>, _version: u32) -> MessageResult<Self>
	where
		T: io::Read,
	{
		let inv = Inv {
			inventory: reader.read_list_max(50_000)?,
		};

		Ok(inv)
	}

	fn serialize_payload(&self, stream: &mut Stream, _version: u32) -> MessageResult<()> {
		stream.append_list(&self.inventory);
		Ok(())
	}
}
