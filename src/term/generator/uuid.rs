#[cfg(any(
	feature = "uuid-generator-v3",
	feature = "uuid-generator-v4",
	feature = "uuid-generator-v5"
))]
use {
	super::{Generator, IriGenerator},
	crate::Id,
};

/// Generates UUID blank node identifiers based on the [`uuid`](https://crates.io/crates/uuid) crate.
///
/// This is an enum type with different UUID versions supported
/// by the `uuid` library, so you can choose which kind of UUID
/// better fits your application.
/// Version 1 is not supported.
///
/// You need to enable the `uuid-generator` feature to
/// use this type.
/// You also need to enable the features of each version you need
/// in the `uuid` crate.
pub enum UuidGenerator {
	/// UUIDv3.
	///
	/// You must provide a vocabulary UUID and a name.
	/// See [uuid::Uuid::new_v3] for more information.
	#[cfg(feature = "uuid-generator-v3")]
	V3(uuid::Uuid, String),

	/// UUIDv4.
	///
	/// See [uuid::Uuid::new_v4] for more information.
	#[cfg(feature = "uuid-generator-v4")]
	V4,

	/// UUIDv5.
	///
	/// You must provide a vocabulary UUID and a name.
	/// See [uuid::Uuid::new_v5] for more information.
	#[cfg(feature = "uuid-generator-v5")]
	V5(uuid::Uuid, String),
}

#[cfg(any(
	feature = "uuid-generator-v3",
	feature = "uuid-generator-v4",
	feature = "uuid-generator-v5"
))]
impl UuidGenerator {
	/// Generates the next UUID.
	pub fn next_uuid(&self) -> uuid::Uuid {
		match self {
			#[cfg(feature = "uuid-generator-v3")]
			Self::V3(vocabulary, name) => uuid::Uuid::new_v3(vocabulary, name.as_bytes()),
			#[cfg(feature = "uuid-generator-v4")]
			Self::V4 => uuid::Uuid::new_v4(),
			#[cfg(feature = "uuid-generator-v5")]
			Self::V5(vocabulary, name) => uuid::Uuid::new_v5(vocabulary, name.as_bytes()),
		}
	}
}

#[cfg(any(
	feature = "uuid-generator-v3",
	feature = "uuid-generator-v4",
	feature = "uuid-generator-v5"
))]
impl IriGenerator for UuidGenerator {
	fn next_iri(&mut self) -> iref::IriBuf {
		let mut buffer: Vec<u8> = vec![0; uuid::adapter::Urn::LENGTH];
		let uuid = self.next_uuid();
		let len = uuid.to_urn().encode_lower(buffer.as_mut()).len();
		buffer.truncate(len);
		iref::IriBuf::from_bytes(buffer).unwrap()
	}
}

#[cfg(any(
	feature = "uuid-generator-v3",
	feature = "uuid-generator-v4",
	feature = "uuid-generator-v5"
))]
impl Generator for UuidGenerator {
	fn next_id(&mut self) -> Id {
		Id::Iri(self.next_iri())
	}
}

#[cfg(any(
	feature = "uuid-generator-v3",
	feature = "uuid-generator-v4",
	feature = "uuid-generator-v5"
))]
#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(feature = "uuid-generator-v3")]
	#[test]
	fn uuidv3_iri() {
		let mut uuid_gen = UuidGenerator::V3(
			uuid::Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap(),
			"test".to_string(),
		);
		for _ in 0..100 {
			use crate::generator::IriGenerator;

			let reference = uuid_gen.next_iri();
			assert!(iref::Iri::new(reference.as_str()).is_ok())
		}
	}

	#[cfg(feature = "uuid-generator-v4")]
	#[test]
	fn uuidv4_iri() {
		let mut uuid_gen = UuidGenerator::V4;
		for _ in 0..100 {
			use crate::generator::IriGenerator;

			let reference = uuid_gen.next_iri();
			assert!(iref::Iri::new(reference.as_str()).is_ok())
		}
	}

	#[cfg(feature = "uuid-generator-v5")]
	#[test]
	fn uuidv5_iri() {
		let mut uuid_gen = UuidGenerator::V5(
			uuid::Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap(),
			"test".to_string(),
		);
		for _ in 0..100 {
			use crate::generator::IriGenerator;

			let reference = uuid_gen.next_iri();
			assert!(iref::Iri::new(reference.as_str()).is_ok())
		}
	}
}
