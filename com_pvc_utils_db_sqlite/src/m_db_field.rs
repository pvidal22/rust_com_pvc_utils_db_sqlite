pub struct SDBField
{
    id: String,
    field_type: String, 
}

impl SDBField
{
    pub fn new(id: &str, field_type: &str) -> Self
    {
        SDBField
        { id: id.to_owned(), field_type: field_type.to_lowercase().to_owned() }
    }

    pub fn get_id(&self) -> &str
    {
        &self.id
    }

    pub fn get_field_type(&self) -> &str
    {
        &self.field_type
    }
}