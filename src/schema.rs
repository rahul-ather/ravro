//! Contains declaration of a struct repr of the Schema type

use std::io::{Write, Read};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::path::Path;
use rand::thread_rng;
use rand::Rng;
use serde_json::{Value, from_reader};
use codec::{Codec, EncodeErr, DecodeErr};
use datafile::SyncMarker;
use types::{Schema, DecodeValue};
use std::fs::File;
use std::str;

pub struct AvroSchema(pub Value);
impl AvroSchema {
	pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
		let schema_file = OpenOptions::new().read(true).open(path).unwrap();
		let file_json_obj = from_reader(schema_file).unwrap();
		Ok(AvroSchema(file_json_obj))
	}

	pub fn as_str(&self) -> Option<&str> {
		self.0.as_str()
	}
}

#[test]
fn test_parse_double_encoded() {
	use std::rc::Rc;
	let mut f = OpenOptions::new().read(true).open("tests/encoded/double_encoded.avro").unwrap();
	let mut magic_buf = [0u8;4];
	f.read_exact(&mut magic_buf[..]).unwrap();
	let decoded_magic = str::from_utf8(&magic_buf[..]).unwrap();
	// Assert header is present
	assert_eq!("Obj\u{1}", decoded_magic);
	// Assert that we have a valid schema
	let map = Schema::decode(&mut f, DecodeValue::Double).unwrap();
	// Pull sync marker for this encoded file
	let sync_marker = SyncMarker::decode(&mut f, DecodeValue::SyncMarker);
	assert!(sync_marker.is_ok());
}
