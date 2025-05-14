use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

mod converters;
mod hocon_parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigFormat {
    Ini,
    Xml,
    Hocon,
    Env,
    Json,
    Yaml,
    Toml,
}

impl ConfigFormat {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "ini" => Some(ConfigFormat::Ini),
            "xml" => Some(ConfigFormat::Xml),
            "conf" | "hocon" => Some(ConfigFormat::Hocon),
            "env" => Some(ConfigFormat::Env),
            "json" => Some(ConfigFormat::Json),
            "yaml" | "yml" => Some(ConfigFormat::Yaml),
            "toml" => Some(ConfigFormat::Toml),
            _ => None,
        }
    }

    pub fn to_extension(&self) -> &'static str {
        match self {
            ConfigFormat::Ini => "ini",
            ConfigFormat::Xml => "xml",
            ConfigFormat::Hocon => "conf",
            ConfigFormat::Env => "env",
            ConfigFormat::Json => "json",
            ConfigFormat::Yaml => "yaml",
            ConfigFormat::Toml => "toml",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigValue {
    #[serde(flatten)]
    pub value: HashMap<String, serde_json::Value>,
}

pub trait ConfigConverter {
    fn parse(&self, content: &str) -> Result<ConfigValue>;
    fn format(&self, config: &ConfigValue) -> Result<String>;
}

pub struct ConfigConverterFactory;

impl ConfigConverterFactory {
    pub fn get_converter(format: ConfigFormat) -> Box<dyn ConfigConverter> {
        match format {
            ConfigFormat::Ini => Box::new(IniConverter),
            ConfigFormat::Xml => Box::new(XmlConverter),
            ConfigFormat::Hocon => Box::new(HoconConverter),
            ConfigFormat::Env => Box::new(EnvConverter),
            ConfigFormat::Json => Box::new(JsonConverter),
            ConfigFormat::Yaml => Box::new(YamlConverter),
            ConfigFormat::Toml => Box::new(TomlConverter),
        }
    }
}

// 各种格式的转换器实现
pub struct IniConverter;
pub struct XmlConverter;
pub struct HoconConverter;
pub struct EnvConverter;
pub struct JsonConverter;
pub struct YamlConverter;
pub struct TomlConverter;

impl ConfigConverter for crate::HoconConverter {
    fn parse(&self, content: &str) -> Result<ConfigValue> {
        let value = hocon_parser::parse_hocon(content)?;
        Ok(ConfigValue { value })
    }

    fn format(&self, config: &ConfigValue) -> Result<String> {
        let mut output = String::new();
        format_hocon_value(&mut output, &config.value, 0)?;
        Ok(output)
    }
}

fn format_hocon_value(output: &mut String, value: &HashMap<String, Value>, indent: usize) -> Result<()> {
    let indent_str = "  ".repeat(indent);
    
    for (key, value) in value {
        output.push_str(&format!("{}{} = ", indent_str, key));
        
        match value {
            Value::Object(obj) => {
                output.push_str("{\n");
                let map: HashMap<_, _> = obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                format_hocon_value(output, &map, indent + 1)?;
                output.push_str(&format!("{}}}\n", indent_str));
            }
            Value::Array(arr) => {
                output.push_str("[\n");
                for item in arr {
                    output.push_str(&format!("{}  ", indent_str));
                    format_hocon_item(output, item)?;
                    output.push_str(",\n");
                }
                output.push_str(&format!("{}]\n", indent_str));
            }
            Value::String(s) => {
                output.push_str(&format!("\"{}\"\n", s));
            }
            Value::Number(n) => {
                output.push_str(&format!("{}\n", n));
            }
            Value::Bool(b) => {
                output.push_str(&format!("{}\n", b));
            }
            Value::Null => {
                output.push_str("null\n");
            }
        }
    }
    
    Ok(())
}

fn format_hocon_item(output: &mut String, value: &Value) -> Result<()> {
    match value {
        Value::Object(obj) => {
            output.push_str("{\n");
            let map: HashMap<_, _> = obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            format_hocon_value(output, &map, 1)?;
            output.push_str("}");
        }
        Value::Array(arr) => {
            output.push_str("[");
            for (i, item) in arr.iter().enumerate() {
                if i > 0 {
                    output.push_str(", ");
                }
                format_hocon_item(output, item)?;
            }
            output.push_str("]");
        }
        Value::String(s) => {
            output.push_str(&format!("\"{}\"", s));
        }
        Value::Number(n) => {
            output.push_str(&n.to_string());
        }
        Value::Bool(b) => {
            output.push_str(&b.to_string());
        }
        Value::Null => {
            output.push_str("null");
        }
    }
    Ok(())
} 