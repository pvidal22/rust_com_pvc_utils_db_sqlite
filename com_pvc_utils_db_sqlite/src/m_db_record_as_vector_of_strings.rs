use std::ops::Deref;
use crate::m_traits::TToVectorOfString;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SRecordAsVectorOfStrings
{
    fields: Vec<String>
}

impl Deref for SRecordAsVectorOfStrings
{
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target 
    {
        &self.fields
    }
}

impl SRecordAsVectorOfStrings
{
    pub fn new(record: Vec<String>) -> Self
    {
        SRecordAsVectorOfStrings
        {
            fields: record,
        }
    }
}

impl TToVectorOfString for SRecordAsVectorOfStrings
{
    fn to_vector_of_strings(&self) -> Vec<String> 
    {
        self.fields.to_owned()
    }
}