use crate::m_db_field::SDBField;

pub type TypeDBRowOfStrings = Vec<String>;

#[derive(Default, Clone, Debug)]
pub struct SDBQueryReturn
{
    fields: Vec<SDBField>,
    records: Vec<TypeDBRowOfStrings>,
    number_of_columns: usize,
}

impl SDBQueryReturn
{
    pub fn new(fields: Vec<SDBField>, records: Vec<TypeDBRowOfStrings>, number_of_columns: usize) -> Self
    {
        SDBQueryReturn
        {
            fields,
            records,
            number_of_columns,
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

    pub fn set_number_columns(&mut self, number_of_columns: usize)
    {
        self.number_of_columns = number_of_columns;
    }

    pub fn get_records(&self) -> &Vec<TypeDBRowOfStrings>
    {
        &self.records
    }

    pub fn get_fields(&self) -> &Vec<SDBField>
    {
        &self.fields
    }

    pub fn get_number_of_columns(&self) -> usize
    {
        self.number_of_columns
    }

    pub fn get_number_records(&self) -> usize
    {
        self.get_records().len()
    }

    pub fn get_record(&self, record_idx: usize) -> Option<&TypeDBRowOfStrings>
    {
        self.get_records().get(record_idx)
    }

    /// Method to retrieve the field value by the field id(column number)
    /// records and fields start counting from zero.
    pub fn get_field_value_by_id(&self, record_idx: usize, field_idx: usize) -> Option<String>
    {
        let record = self.get_record(record_idx)?;
        let field = record.get(field_idx)?;
        Some(field.to_owned())
    }

    /// Method to retrieve the field value by the field_name
    /// records and fields start counting from zero.
    pub fn get_field_value_by_name(&self, record_idx: usize, field_name: &str) -> Option<String>
    {
        let record = self.get_record(record_idx)?;
        let field_idx = self.get_field_by_field_name(field_name)?.get_idx();
        let field = record.get(field_idx)?;
        Some(field.to_owned())
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
