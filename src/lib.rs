// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }


pub mod JsonParser
{
    use std::collections::HashMap;
    use std::fs;
    use std::error::Error;
    
    pub enum JsonValue
    {
        Map(HashMap<String, JsonValue>),
        Vector(Vec<JsonValue>),
        Bool(bool),
        Double(f32),
        String(String),
        Null,
    }

    fn getFileContent(file_path: String) -> Result<String, Box<dyn Error>>
    {
        let file_content = fs::read_to_string(file_path)?;
        Ok(file_content)
    }

    fn parseMap(file_path: String) -> Result<JsonValue, &'static str>
    {

    }

    fn parseVec(file_path: String) -> Result<JsonValue, &'static str>
    {
        
    }

    pub fn parse(file_path: String) -> Result<JsonValue, Box<dyn Error>>
    {
        let mut file_content = getFileContent(file_path)?;
        file_content.trim();

        let mut json_obj: JsonValue;

        if file_content.starts_with("{")
        {
            json_obj = parseMap(file_content)?;
        }
        else if file_content.starts_with("[")
        {
            json_obj = parseVec(file_content)?;
        }

        Ok(json_obj)
    }
}