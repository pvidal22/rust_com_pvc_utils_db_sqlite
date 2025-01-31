use crate::m_db_field::SDBField;

pub trait TToVectorOfString
{
    fn to_vector_of_strings(&self) -> Vec<String>;
}

#[derive(Default, Clone, Debug)]
pub struct SDBQueryReturn<T>
{
    fields: Vec<SDBField>,
    records: Vec<T>,
}

impl<T> SDBQueryReturn<T>
where T: TToVectorOfString
{
    pub fn new(fields: Vec<SDBField>, records: Vec<T>) -> Self
    {
        SDBQueryReturn
        {
            fields,
            records,
        }
    }

    pub fn new_only_fields(fields: Vec<SDBField>) -> Self
    {
        SDBQueryReturn
        {
            fields,
            records: Vec::new(),
        }
    }
    
    pub fn set_fields(&mut self, fields: Vec<SDBField>)
    {
        self.fields = fields;
    }

    pub fn set_records(&mut self, records: Vec<T>)
    {
        self.records = records;
    }

    pub fn get_records(&self) -> &Vec<T>
    {
        &self.records
    }

    pub fn get_fields(&self) -> &Vec<SDBField>
    {
        &self.fields
    }

    pub fn get_number_of_columns(&self) -> usize
    {
        self.fields.len()
    }

    pub fn get_number_of_records(&self) -> usize
    {
        self.get_records().len()
    }

    pub fn get_record(&self, record_idx: usize) -> Option<&T>
    {
        self.get_records().get(record_idx)
    }

    /// Method to retrieve the field value by the field id(column number)
    /// records and fields start counting from zero.
    /// function only valid when the QueryReturn is made out of Vec<Strings>
    pub fn get_field_value_by_id(&self, record_idx: usize, field_idx: usize) -> Option<String>
    {
        let record = self.get_record(record_idx)?;
        let record = record.to_vector_of_strings();
        Some(record.get(field_idx)?.to_owned())
    }

    /// Method to retrieve the field value by the field_name
    /// records and fields start counting from zero.
    /// function only valid when the QueryReturn is made out of Vec<Strings>
    pub fn get_field_value_by_name(&self, record_idx: usize, field_name: &str) -> Option<String>
    {
        let field_idx = self.get_field_by_field_name(field_name)?.get_idx();
        self.get_field_value_by_id(record_idx, field_idx)
    }

    pub fn get_field_by_index(&self, idx: usize) -> Option<&SDBField>
    {
        let fields = self.get_fields();
        if idx >= idx
            {return None};
        fields.get(idx)
    }

    pub fn get_field_by_field_name(&self, field_name: &str) -> Option<&SDBField>
    {
        let fields = self.get_fields();
        fields.iter().find(|x| x.get_name() == field_name)
    }
}
