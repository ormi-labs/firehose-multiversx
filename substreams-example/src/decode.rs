use crate::utils::Field;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Split;
use substreams::scalar::BigInt;

pub struct TxData<'a> {
    method: &'a str,
    segments: Peekable<Split<'a, char>>,
}

pub fn parse_data<'a>(segments: Split<'a, char>, method: &'a str, cb: impl FnOnce(TxData)) {
    cb(TxData {
        method,
        segments: segments.peekable(),
    });
}

impl TxData<'_> {
    /// Decodes the next field as a string
    pub fn field_string(&mut self, field_name: &'static str) -> Field {
        Field::from_tovalue(field_name, self.next_utf8(field_name))
    }

    /// Returns the next field as a raw hex string
    pub fn field_raw(&mut self, field_name: &'static str) -> Field {
        Field::from_tovalue(field_name, self.next_raw(field_name))
    }

    /// Returns the next field as an unsigned bigint
    pub fn field_bigint(&mut self, field_name: &'static str) -> Field {
        Field::from_tovalue(
            field_name,
            BigInt::from_unsigned_bytes_be(&self.next_decoded(field_name)),
        )
    }

    /// Decodes the rest of the data as extra fields
    pub fn extra_fields(self) -> Field {
        let extra_fields = self
            .segments
            .map(|s| {
                String::from_utf8(hex::decode(s).expect("Should decode other fields")).unwrap()
            })
            .chunks(2)
            .into_iter()
            .map(|mut c| {
                let key = c.next().expect("Chunk should not be empty");
                let value = c
                    .next()
                    .unwrap_or_else(|| panic!("Extra field `{key}` is missing a value"));
                (key, value)
            })
            .collect::<HashMap<_, _>>();

        Field::from_tovalue(
            "extra_fields",
            nanoserde::SerJson::serialize_json(&extra_fields),
        )
    }

    /// Returns the raw next segment
    pub fn next_raw(&mut self, field_name: &'static str) -> &str {
        self.segments
            .next()
            .unwrap_or_else(|| panic!("Field {} is missing in `{}` data", field_name, self.method))
    }

    /// Returns the hex-decoded next segment
    pub fn next_decoded(&mut self, field_name: &'static str) -> Vec<u8> {
        hex::decode(self.next_raw(field_name)).unwrap_or_else(|_| {
            panic!(
                "Field {} contains invalid hex in `{}` data",
                field_name, self.method
            )
        })
    }

    /// Returns hex-utf8-decoded hex segment
    pub fn next_utf8(&mut self, field_name: &'static str) -> String {
        String::from_utf8(self.next_decoded(field_name)).unwrap_or_else(|_| {
            panic!(
                "Field {} contains invalid UTF `{}` data",
                field_name, self.method
            )
        })
    }

    pub fn has_next(&mut self) -> bool {
        self.segments.peek().is_some()
    }
}
