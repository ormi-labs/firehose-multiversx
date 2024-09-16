use substreams_entity_change::pb::entity::Value;
use substreams_entity_change::tables::{Row, ToValue};

#[derive(Debug, PartialEq)]
pub struct ValueHolder(Value);

impl ValueHolder {
    pub fn new(value: impl ToValue) -> Self {
        Self(value.to_value())
    }
}

impl From<Value> for ValueHolder {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

impl ToValue for ValueHolder {
    fn to_value(self) -> Value {
        self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: &'static str,
    pub value: ValueHolder,
}

impl Field {
    pub fn new(name: &'static str, value: ValueHolder) -> Self {
        Self { name, value }
    }

    pub fn from_tovalue(name: &'static str, value: impl ToValue) -> Self {
        Self {
            name,
            value: ValueHolder(value.to_value()),
        }
    }
}

pub trait RowExt {
    fn set_field(&mut self, field: Field) -> &mut Self;
}

impl RowExt for Row {
    fn set_field(&mut self, field: Field) -> &mut Self {
        self.set(field.name, field.value);
        self
    }
}
