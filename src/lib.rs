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
        fn insert_map(&mut self, key: String, value: JsonValue) -> Result<(), &'static str>
        {
            match self
            {
                JsonValue::Dict(map) =>
                {
                    map.insert(key, value);
                    Ok(())
                },
                _ => Err("Cannot insert in a map if JsonValue is not Dict type")
            }
        }

        fn insert_vec(&mut self, value: JsonValue) -> Result<(), &'static str>
        {
            match self
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

    fn parseValue(mut file_content: String, end_char: char) -> Result<(String, JsonValue), &'static str>
    {
        match file_content.as_bytes()[0]
        {
            b'{' =>
            {
                let mut value = JsonValue::Dict(HashMap::new());
                (file_content, value) = parseMap(file_content)?;
                Ok((file_content, value))
            }
            b'[' =>
            {
                let mut value = JsonValue::List(Vec::new());
                (file_content, value) = parseVec(file_content)?;
                Ok((file_content, value))
            }
            _ =>
            {
                let mut value_str = String::new();
                while file_content.len() > 0 && !file_content.starts_with(",") && !file_content.starts_with(end_char)
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
                        Ok((file_content, JsonValue::Null))
                    },
                    "true" =>
                    {
                        Ok((file_content, JsonValue::Bool(true)))
                    }
                    "false" =>
                    {
                        Ok((file_content, JsonValue::Bool(false)))
                    }
                    _ =>
                    {
                        if value_str.as_bytes()[0] == b'\'' || value_str.as_bytes()[0] == b'\"'
                        {
                            let value = JsonValue::String(parseStrValue(value_str)?);
                            Ok((file_content, value))
                        }
                        else
                        {
                            match value_str.parse::<i64>()
                            {
                                Ok(v) => 
                                {
                                    Ok((file_content, JsonValue::Int(v)))
                                },
                                Err(_) =>
                                {
                                    match value_str.parse::<f64>()
                                    {
                                        Ok(v) =>
                                        {
                                            Ok((file_content, JsonValue::Double(v)))
                                        },
                                        Err(_) =>
                                        {
                                            Err("Wrong value format")
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn parseMap(mut file_content: String) -> Result<(String, JsonValue), &'static str>
    {
        let mut return_map = JsonValue::Dict(HashMap::new());
        file_content.remove(0);

        loop
        {
            let mut key = String::new();
            (file_content, key) = parseKey(file_content)?;

            let mut value = JsonValue::Null;
            (file_content, value) = parseValue(file_content, '}')?;

            return_map.insert_map(key, value);

            match file_content.remove(0)
            {
                ',' => {},
                '}' => 
                {
                    break;
                },
                _ =>
                {
                    return Err("Wrong format for type Dict");
                }
            }
        }

        Ok((file_content, return_map))
    }

    fn parseVec(mut file_content: String) -> Result<(String, JsonValue), &'static str>
    {
        let mut return_vec = JsonValue::List(Vec::new());
        file_content.remove(0);

        loop
        {
            let mut value = JsonValue::Null;
            (file_content, value) = parseValue(file_content, ']')?;

            return_vec.insert_vec(value);

            match file_content.remove(0)
            {
                ',' => {},
                ']' => 
                {
                    break;
                },
                _ =>
                {
                    return Err("Wrong format for type List");
                }
            }
        }

        Ok((file_content, return_vec))
    }

    pub fn parse(file_path: String) -> Result<JsonValue, Box<dyn Error>>
    {
        let mut file_content = getFileContent(file_path)?;

        let json_obj: JsonValue;

        match file_content.as_bytes()[0]
        {
            b'{' =>
            {
                (file_content, json_obj) = parseMap(file_content)?;
            }
            b'[' =>
            {
                (file_content, json_obj) = parseVec(file_content)?;
            }
            _ =>
            {
                return Err("Wrong Json format".into());
            }
        }

        Ok(json_obj)
    }
}