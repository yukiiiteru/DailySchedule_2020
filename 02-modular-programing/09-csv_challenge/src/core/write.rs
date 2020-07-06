use super::*;

pub fn replace_column(data: String, column: &str, replacement: &str)
    -> Result<String, Error> {
    let mut lines = data.lines();
    let headers = lines.next().unwrap();
    let columns: Vec<&str> = headers.split(',').collect();
    let column_number = columns.iter().position(|&e| e == column);
    let column_number = match column_number {
        Some(column)    => column,
        None            => Err("column name doesn't exist in the input file")?
    };
    let mut result = String::with_capacity(data.capacity());
    result.push_str(&columns.join(","));
    result.push('\n');
    for line in lines {
        let mut records: Vec<&str> = line.split(',').collect();
        records[column_number] = replacement;
        result.push_str(&records.join(","));
        result.push('\n');
    }
    Ok(result)
}
