use std::fs;
use toml_edit::{DocumentMut, Item, value};
use anyhow::{Result, anyhow};

pub fn get_value(path: &str, key: &str) -> Result<String> {
    let toml_str = fs::read_to_string(path)?;
    let doc = toml_str.parse::<DocumentMut>()?;

    let mut current_item: &Item = &Item::Table(doc.as_table().clone().into());
    for k in key.split('.') {
        current_item = match current_item.get(k) {
            Some(v) => v,
            None => return Err(anyhow!("{} not found", k)),
        };
    }

    let result = if let Item::Value(v) = current_item {
        if v.is_str() {
            v.as_str().unwrap().to_string()
        } else {
            v.to_string()
        }
    } else {
        current_item.to_string()
    };

    Ok(result.trim().to_string())
}

pub fn set_value(path: &str, key: &str, new_value: &str) -> Result<()> {
    let toml_str = fs::read_to_string(path)?;
    let mut doc = toml_str.parse::<DocumentMut>()?;

    let keys: Vec<&str> = key.split('.').collect();
    if keys.is_empty() {
        return Err(anyhow!("key is empty"));
    }

    let mut current_item: &mut Item = doc.as_item_mut();
    for k in &keys[..keys.len()-1] {
       let table: &mut toml_edit::Table = current_item.as_table_mut()
        .ok_or_else(|| anyhow!("{} is not a table", k))?;

        if !table.contains_key(*k) {
            table[k] = Item::Table(toml_edit::Table::new());
        }
        current_item = table.get_mut(*k)
            .ok_or_else(|| anyhow!("{} not found", k))?;
    }

    let last_key = keys[keys.len() - 1];
    let table = current_item
        .as_table_mut()
        .ok_or_else(|| anyhow!("{} is not a table", last_key))?;

    let old_item = table.get(last_key)
        .ok_or_else(|| anyhow!("{} not found", last_key))?;

    let new_item = if old_item.is_integer() {
        value(new_value.parse::<i64>()?)
    } else if old_item.is_float() {
        value(new_value.parse::<f64>()?)
    } else if old_item.is_bool() {
        value(new_value.parse::<bool>()?)
    } else {
        value(new_value)
    };

    table[last_key] = new_item;

    fs::write(path, doc.to_string())?;
    Ok(())
}

#[test]
fn test_get_value() {
    println!("{}", get_value("frpc.toml", "serverPort").unwrap());
}

#[test]
fn test_set_value() {
    set_value("frpc.toml", "serverPort", "7001").unwrap();
}