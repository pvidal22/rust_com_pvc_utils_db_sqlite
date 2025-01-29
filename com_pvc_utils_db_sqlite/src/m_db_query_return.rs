use rusqlite::types::Type;

use crate::m_db_field::SDBField;

pub type TypeDBRowOfStrings = Vec<String>;

#[derive(Default, Clone, Debug)]
pub struct SDBQueryReturn
{
    fields: Vec<SDBField>,
    records: Vec<TypeDBRowOfStrings>
}

impl SDBQueryReturn
{
    pub fn new(fields: Vec<SDBField>, records: Vec<TypeDBRowOfStrings>) -> Self
    {
        SDBQueryReturn
        {
            fields,
            records,
        }
    }

    pub fn set_fields(&mut self, fields: Vec<SDBField>)
    {
        self.fields = fields;
    }

    pub fn set_records(&mut self, records: Vec<TypeDBRowOfStrings>)
    {
        self.records = records;
    }

    pub fn get_records(&self) -> &Vec<TypeDBRowOfStrings>
    {
        &self.records
    }

    pub fn get_fields(&self) -> &Vec<SDBField>
    {
        &self.fields
    }

    pub fn get_field_by_index(&self, idx: usize) -> Option<&SDBField>
    {
        let fields = self.get_fields();
        if idx >= idx
            {return None};
        fields.get(idx)
    }

    pub fn get_field_by_id(&self, id: &str) -> Option<&SDBField>
    {
        let fields = self.get_fields();
        fields.iter().find(|x| x.get_id() == id)
    }
}
