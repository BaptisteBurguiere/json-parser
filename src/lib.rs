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
        Dict(HashMap<String, JsonValue>),
        List(Vec<JsonValue>),
        Bool(bool),
        Int(i64),
        Double(f64),
        String(String),
        Null,
    }

    impl JsonValue
    {
        fn insert_map(&self, key: String, value: JsonValue) -> Result<(), &'static str>
        {
            match &self
            {
                JsonValue::Dict(map) =>
                {
                    map.insert(key, value);
                    Ok(())
                },
                _ => Err("Cannot insert in a map if JsonValue is not Dict type")
            }
        }

        fn insert_vec(&self, value: JsonValue) -> Result<(), &'static str>
        {
            match &self
            {
                JsonValue::List(vec) =>
                {
                    vec.push(value);
                    Ok(())
                },
                _ => Err("Cannot insert in a vector if JsonValue is not List type")
            }
        }
    }

    fn getFileContent(file_path: String) -> Result<String, Box<dyn Error>>
    {
        let mut file_content = fs::read_to_string(file_path)?;
        file_content.trim();

        let mut return_str = String::new();
        while file_content.len() > 0
        {
            if file_content.starts_with("\"")
            {
                return_str.push(file_content.remove(0));
                while !file_content.starts_with("\"")
                {
                    return_str.push(file_content.remove(0));
                }
                return_str.push(file_content.remove(0));
            }
            else if file_content.starts_with("\'")
            {
                return_str.push(file_content.remove(0));
                while !file_content.starts_with("\'")
                {
                    return_str.push(file_content.remove(0));
                }
                return_str.push(file_content.remove(0));
            }
            else if file_content.starts_with(" ") || file_content.starts_with("\t") || file_content.starts_with("\n")
            {
                file_content.remove(0);
            }
            else
            {
                return_str.push(file_content.remove(0))
            }
        }
        
        Ok(return_str)
    }

    fn parseKey(mut file_content: String) -> Result<(String, String), &'static str>
    {
        match file_content.remove(0)
        {
            '\"' =>
            {
                let mut key = String::new();
                while file_content.len() > 0 && file_content.starts_with("\"")
                {
                    key.push(file_content.remove(0));
                }
                if file_content.len() > 0
                {
                    file_content.remove(0);
                }

                if file_content.len() == 0 || !file_content.starts_with(":")
                {
                    return Err("Wrong key format");
                }
                file_content.remove(0);
                Ok((file_content, key))
            }
            '\'' =>
            {
                let mut key = String::new();
                while file_content.len() > 0 && file_content.starts_with("\'")
                {
                    key.push(file_content.remove(0));
                }
                if file_content.len() > 0
                {
                    file_content.remove(0);
                }
    
                if file_content.len() == 0 || !file_content.starts_with(":")
                {
                    return Err("Wrong key format");
                }
                file_content.remove(0);
                Ok((file_content, key))
            }
            _ =>
            {
                Err("Wrong key format")
            }
        }
    }

    fn parseStrValue(mut value_str: String) -> Result<String, &'static str>
    {
        match value_str.remove(0)
        {
            '\"' =>
            {
                let mut value = String::new();
                while value_str.len() > 0 && !value_str.starts_with('\"')
                {
                    value.push(value_str.remove(0));
                }
                if value_str.len() == 0
                {
                    return Err("Wrong format for str value");
                }
                Ok(value)
            }
            '\'' =>
            {
                let mut value = String::new();
                while value_str.len() > 0 && !value_str.starts_with('\'')
                {
                    value.push(value_str.remove(0));
                }
                if value_str.len() == 0
                {
                    return Err("Wrong format for str value");
                }
                Ok(value)
            }
            _ =>
            {
                Err("Wrong format for str value")
            }
        }
    }

    fn parseMap(mut file_content: String) -> Result<(String, JsonValue), &'static str>
    {
        let return_map = JsonValue::Dict(HashMap::new());
        file_content.remove(0);

        loop
        {
            let mut key = String::new();
            (file_content, key) = parseKey(file_content)?;

            match file_content.as_bytes()[0]
            {
                b'{' =>
                {
                    let value = JsonValue::Dict(HashMap::new());
                    return_map.insert_map(key, value);
                }
                b'[' =>
                {
                    let value = JsonValue::List(Vec::new());
                    return_map.insert_map(key, value);
                }
                _ =>
                {
                    let mut value_str = String::new();
                    while file_content.len() > 0 && !file_content.starts_with(",") && !file_content.starts_with("}")
                    {
                        value_str.push(file_content.remove(0));
                    }

                    if file_content.len() == 0
                    {
                        return Err("Wrong Dict format");
                    }

                    match value_str.as_str()
                    {
                        "null" =>
                        {
                            return_map.insert_map(key, JsonValue::Null);
                        },
                        "true" =>
                        {
                            return_map.insert_map(key, JsonValue::Bool(true));
                        }
                        "false" =>
                        {
                            return_map.insert_map(key, JsonValue::Bool(false));
                        }
                        _ =>
                        {
                            if value_str.as_bytes()[0] == b'\'' || value_str.as_bytes()[0] == b'\"'
                            {
                                let value = JsonValue::String(parseStrValue(value_str)?);
                                return_map.insert_map(key, value);
                            }
                            else
                            {
                                match value_str.parse::<i64>()
                                {
                                    Ok(v) => 
                                    {
                                        return_map.insert_map(key, JsonValue::Int(v));
                                    },
                                    Err(_) =>
                                    {
                                        match value_str.parse::<f64>()
                                        {
                                            Ok(v) =>
                                            {
                                                return_map.insert_map(key, JsonValue::Double(v));
                                            },
                                            Err(_) =>
                                            {
                                                return Err("Wrong value format");
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    match file_content.remove(0)
                    {
                        ',' => {},
                        '}' => 
                        {
                            break;
                        }
                    }
                }
            }
        }

        Ok((file_content, return_map))
    }

    fn parseVec(file_content: String) -> Result<(String, JsonValue), &'static str>
    {
        
    }

    pub fn parse(file_path: String) -> Result<JsonValue, Box<dyn Error>>
    {
        let mut file_content = getFileContent(file_path)?;

        let mut json_obj: JsonValue;

        if file_content.starts_with("{")
        {
            (file_content, json_obj) = parseMap(file_content)?;
        }
        else if file_content.starts_with("[")
        {
            (file_content, json_obj) = parseVec(file_content)?;
        }

        Ok(json_obj)
    }
}