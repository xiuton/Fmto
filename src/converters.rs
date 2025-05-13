use crate::{ConfigConverter, ConfigValue};
use anyhow::{anyhow, Result};
use configparser::ini::Ini;
use serde_json::Value;
use std::collections::HashMap;

impl ConfigConverter for crate::IniConverter {
    fn parse(&self, content: &str) -> Result<ConfigValue> {
        let mut ini = Ini::new();
        ini.read(content.to_string()).map_err(|e| anyhow::anyhow!("INI 解析错误: {}", e))?;
        
        let mut value = HashMap::new();
        for section in ini.sections() {
            let mut section_map = HashMap::new();
            let props = ini.get(&section, "").unwrap_or_default();
            for key in props.split('\n') {
                if let Some((k, v)) = key.split_once('=') {
                    section_map.insert(k.trim().to_string(), Value::String(v.trim().to_string()));
                }
            }
            value.insert(section, Value::Object(section_map.into_iter().collect()));
        }
        
        Ok(ConfigValue { value })
    }

    fn format(&self, config: &ConfigValue) -> Result<String> {
        let mut ini = Ini::new();
        
        for (section, value) in &config.value {
            if let Value::Object(props) = value {
                for (key, val) in props {
                    if let Value::String(s) = val {
                        ini.set(section, key, Some(s.clone()));
                    }
                }
            }
        }
        
        Ok(ini.writes())
    }
}

impl ConfigConverter for crate::JsonConverter {
    fn parse(&self, content: &str) -> Result<ConfigValue> {
        let value: HashMap<String, Value> = serde_json::from_str(content)?;
        Ok(ConfigValue { value })
    }

    fn format(&self, config: &ConfigValue) -> Result<String> {
        Ok(serde_json::to_string_pretty(&config.value)?)
    }
}

impl ConfigConverter for crate::YamlConverter {
    fn parse(&self, content: &str) -> Result<ConfigValue> {
        let value: HashMap<String, Value> = serde_yaml::from_str(content)?;
        Ok(ConfigValue { value })
    }

    fn format(&self, config: &ConfigValue) -> Result<String> {
        Ok(serde_yaml::to_string(&config.value)?)
    }
}

impl ConfigConverter for crate::TomlConverter {
    fn parse(&self, content: &str) -> Result<ConfigValue> {
        let value: HashMap<String, Value> = toml::from_str(content)?;
        Ok(ConfigValue { value })
    }

    fn format(&self, config: &ConfigValue) -> Result<String> {
        Ok(toml::to_string_pretty(&config.value)?)
    }
}

impl ConfigConverter for crate::EnvConverter {
    fn parse(&self, content: &str) -> Result<ConfigValue> {
        let mut value = HashMap::new();
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some((key, val)) = line.split_once('=') {
                value.insert(key.trim().to_string(), Value::String(val.trim().to_string()));
            }
        }
        
        Ok(ConfigValue { value })
    }

    fn format(&self, config: &ConfigValue) -> Result<String> {
        let mut output = String::new();
        
        for (key, value) in &config.value {
            if let Value::String(s) = value {
                output.push_str(&format!("{}={}\n", key, s));
            }
        }
        
        Ok(output)
    }
}

impl ConfigConverter for crate::XmlConverter {
    fn parse(&self, content: &str) -> Result<ConfigValue> {
        let mut value = HashMap::new();
        let doc = quick_xml::de::from_str::<HashMap<String, Value>>(content)?;
        value.extend(doc);
        Ok(ConfigValue { value })
    }

    fn format(&self, config: &ConfigValue) -> Result<String> {
        Ok(quick_xml::se::to_string(&config.value)?)
    }
} 