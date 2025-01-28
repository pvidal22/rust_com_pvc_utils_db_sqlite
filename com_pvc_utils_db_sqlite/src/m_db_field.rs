#[derive(Clone, Debug)]
pub enum EDBFieldType
{
    Null,
    Integer,
    Real,
    Text,
    Blob,
}

pub struct SDBField
{
    id: String,
    field_type: EDBFieldType, 
}

impl SDBField
{
    pub fn new(id: &str, field_type: EDBFieldType) -> Self
    {
        SDBField
        { id: id.to_owned(), field_type }
    }

    pub fn get_id(&self) -> &str
    {
        &self.id
    }

    pub fn get_field_type(&self) -> &EDBFieldType
    {
        &self.field_type
    }
}