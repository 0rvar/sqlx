use crate::any::value::AnyValueKind;
use crate::any::{Any, AnyValueRef};
use crate::arguments::Arguments;
use crate::encode::Encode;
use crate::types::Type;
use std::marker::PhantomData;

pub struct AnyArguments<'q> {
    values: AnyArgumentBuffer<'q>,
}

impl<'q> Arguments<'q> for AnyArguments<'q> {
    type Database = Any;

    fn reserve(&mut self, additional: usize, _size: usize) {
        self.values.0.reserve(additional);
    }

    fn add<T>(&mut self, value: T)
    where
        T: 'q + Send + Encode<'q, Self::Database> + Type<Self::Database>,
    {
        value.encode(&mut self.values);
    }
}

pub struct AnyArgumentBuffer<'q>(pub(crate) Vec<AnyValueKind<'q>>);

impl<'q> Default for AnyArguments<'q> {
    fn default() -> Self {
        AnyArguments {
            values: AnyArgumentBuffer(vec![]),
        }
    }
}
