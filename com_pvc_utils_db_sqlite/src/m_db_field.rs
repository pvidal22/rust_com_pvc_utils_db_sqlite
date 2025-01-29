#[derive(Clone, Debug, Default)]
pub enum EDBFieldType
{
    Null,
    Integer,
    Real,
    #[default]
    Text,
    Blob,
}

#[derive(Default, Clone, Debug)]
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