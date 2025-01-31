#[derive(Clone, Debug, Default)]
pub enum EDBFieldType
{
    #[default]
    NotYetDefined,
    Null,
    Integer,
    Real,    
    Text,
    Blob,
}

#[derive(Default, Clone, Debug)]
pub struct SDBField
{
    field_idx: usize,
    field_name: String,
    field_type: EDBFieldType, 
}

impl SDBField
{
    pub fn new(field_idx: usize, field_name: &str, field_type: EDBFieldType) -> Self
    {
        SDBField
        { 
            field_idx
            , field_name: field_name.to_owned()
            , field_type 
        }
    }

    pub fn get_idx(&self) -> usize
    {
        self.field_idx
    }

    pub fn get_name(&self) -> &str
    {
        &self.field_name
    }
    
    pub fn get_field_type(&self) -> &EDBFieldType
    {
        &self.field_type
    }
}