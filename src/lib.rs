extern crate serde_yaml;

pub fn run(yaml_data: serde_yaml::Value, format: String) {
    let mut keys_result: Vec<String> = Vec::new();
    let mut values_result: Vec<String> = Vec::new();

    for (key, value) in yaml_data.as_mapping().unwrap() {
        handle_all_root_value_types(key, value, &mut keys_result, &mut values_result)
    }

    display_output(format, keys_result, values_result);
}

fn handle_root_bool_value_type(
    key: &str,
    value: &serde_yaml::Value,
    keys_result: &mut Vec<String>,
    values_result: &mut Vec<String>,
) {
    if value.is_bool() {
        values_result.push(value.as_bool().unwrap().to_string());
        keys_result.push(key.to_string());
    }
}

fn handle_root_string_value_type(
    key: &str,
    value: &serde_yaml::Value,
    keys_result: &mut Vec<String>,
    values_result: &mut Vec<String>,
) {
    if value.is_string() {
        values_result.push(value.as_str().unwrap().to_string());
        keys_result.push(key.to_string());
    }
}

fn handle_root_number_value_type(
    key: &str,
    value: &serde_yaml::Value,
    keys_result: &mut Vec<String>,
    values_result: &mut Vec<String>,
) {
    if value.is_number() {
        if value.is_i64() {
            values_result.push(value.as_i64().unwrap().to_string());
            keys_result.push(key.to_string());
        } else if value.is_f64() {
            values_result.push(value.as_f64().unwrap().to_string());
            keys_result.push(key.to_string());
        }
    }
}

fn handle_root_sequence_value_type_recursively(
    key: &str,
    value: &serde_yaml::Value,
    keys_result: &mut Vec<String>,
    values_result: &mut Vec<String>,
) -> (Vec<String>, Vec<String>) {
    if value.is_sequence() {
        for item in value.as_sequence().unwrap() {
            if item.is_sequence() {
                handle_root_sequence_value_type_recursively(key, item, keys_result, values_result);
            } else if item.is_mapping() {
                handle_root_mapping_value_type_recursively(key, item, keys_result, values_result);
            }
        }
    }

    (keys_result.to_vec(), values_result.to_vec())
}

fn handle_root_mapping_value_type_recursively(
    key: &str,
    map_value: &serde_yaml::Value,
    keys_result: &mut Vec<String>,
    values_result: &mut Vec<String>,
) -> (Vec<String>, Vec<String>) {
    if map_value.is_mapping() {
        for (sub_key, sub_value) in map_value.as_mapping().unwrap() {
            let new_sub_key = format!("{}.{}", key, &sub_key.as_str().unwrap());

            handle_root_bool_value_type(&new_sub_key, sub_value, keys_result, values_result);
            handle_root_number_value_type(&new_sub_key, sub_value, keys_result, values_result);
            handle_root_string_value_type(&new_sub_key, sub_value, keys_result, values_result);
            handle_root_mapping_value_type_recursively(
                &new_sub_key,
                sub_value,
                keys_result,
                values_result,
            );
            handle_root_sequence_value_type_recursively(
                &new_sub_key,
                sub_value,
                keys_result,
                values_result,
            );
        }
    }

    (keys_result.to_vec(), values_result.to_vec())
}

fn handle_all_root_value_types(
    key: &serde_yaml::Value,
    value: &serde_yaml::Value,
    keys_result: &mut Vec<String>,
    values_result: &mut Vec<String>,
) {
    let new_key = key.as_str().unwrap();

    handle_root_bool_value_type(new_key, value, keys_result, values_result);
    handle_root_number_value_type(new_key, value, keys_result, values_result);
    handle_root_string_value_type(new_key, value, keys_result, values_result);
    handle_root_sequence_value_type_recursively(new_key, value, keys_result, values_result);
    handle_root_mapping_value_type_recursively(new_key, value, keys_result, values_result);
}

fn display_output(format: String, keys: Vec<String>, values: Vec<String>) {
    match &format as &str {
        "hcl_map" => print_hcl_map(keys, values), // Terraform varible of type map
        "set_value" => print_helm_release_set_value(keys, values), // Terraform helm_release provider set_value
        "sensitive_value" => print_helm_release_sensitive_set_value(keys, values), // Terraform helm_release provider sensitive_value
        "helm_cli" => print_helm_cli_set_value(keys, values), // Helm Chart Cli set value
        _ => (),
    }
}

fn print_hcl_map(keys: Vec<String>, values: Vec<String>) {
    println!("hcl_map = {{");
    for (key, value) in keys.iter().zip(values.iter()) {
        println!("  \"{}\" = \"{}\"", key, value)
    }
    println!("}}");
}

fn print_helm_release_set_value(keys: Vec<String>, values: Vec<String>) {
    for (key, value) in keys.iter().zip(values.iter()) {
        println!(
            r#"
    set_value = {{
      name = "{}"
      value = "{}"
    }}
    "#,
            key, value
        );
    }
}

fn print_helm_release_sensitive_set_value(keys: Vec<String>, values: Vec<String>) {
    for (key, value) in keys.iter().zip(values.iter()) {
        println!(
            r#"
    set_sensitive = {{
      name = "{}"
      value = "{}"
    }}
    "#,
            key, value
        );
    }
}

fn print_helm_cli_set_value(keys: Vec<String>, values: Vec<String>) {
    for (key, value) in keys.iter().zip(values.iter()) {
        println!("--set {}=\"{}\" \\", key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bool_value_from_yaml() {
        let mut keys_result: Vec<String> = Vec::new();
        let mut values_result: Vec<String> = Vec::new();

        let yaml_input: serde_yaml::Value =
            serde_yaml::from_str("enable_ssl: true").expect("Failed to parse YAML");

        for (key, value) in yaml_input.as_mapping().unwrap() {
            let new_key = key.as_str().unwrap();
            handle_root_bool_value_type(new_key, value, &mut keys_result, &mut values_result);
        }

        println!("Keys: {:?}, Values: {:?}", keys_result, values_result);

        let expected_keys_result = ["enable_ssl"];
        let expected_values_result = ["true"];
        assert_eq!(keys_result, expected_keys_result);
        assert_eq!(values_result, expected_values_result);
    }

    #[test]
    fn parse_string_value_from_yaml() {
        let mut keys_result: Vec<String> = Vec::new();
        let mut values_result: Vec<String> = Vec::new();

        let yaml_input: serde_yaml::Value =
            serde_yaml::from_str("protocol_type: http").expect("Failed to parse YAML");

        for (key, value) in yaml_input.as_mapping().unwrap() {
            let new_key = key.as_str().unwrap();
            handle_root_string_value_type(new_key, value, &mut keys_result, &mut values_result);
        }

        println!("Keys: {:?}, Values: {:?}", keys_result, values_result);

        let expected_keys_result = ["protocol_type"];
        let expected_values_result = ["http"];
        assert_eq!(keys_result, expected_keys_result);
        assert_eq!(values_result, expected_values_result);
    }

    #[test]
    fn parse_i64_value_from_yaml() {
        let mut keys_result: Vec<String> = Vec::new();
        let mut values_result: Vec<String> = Vec::new();

        let yaml_input: serde_yaml::Value =
            serde_yaml::from_str("port: 8080").expect("Failed to parse YAML");

        for (key, value) in yaml_input.as_mapping().unwrap() {
            let new_key = key.as_str().unwrap();
            handle_root_number_value_type(new_key, value, &mut keys_result, &mut values_result);
        }

        println!("Keys: {:?}, Values: {:?}", keys_result, values_result);
        println!("{:?}", values_result[0].parse::<i64>().unwrap());

        let expected_keys_result = ["port"];
        let expected_values_result = ["8080"];
        assert_eq!(keys_result, expected_keys_result);
        assert_eq!(values_result, expected_values_result);
    }

    #[test]
    fn parse_f64_value_from_yaml() {
        let mut keys_result: Vec<String> = Vec::new();
        let mut values_result: Vec<String> = Vec::new();

        let yaml_input: serde_yaml::Value =
            serde_yaml::from_str("some_price: 200.44").expect("Failed to parse YAML");

        for (key, value) in yaml_input.as_mapping().unwrap() {
            let new_key = key.as_str().unwrap();
            handle_root_number_value_type(new_key, value, &mut keys_result, &mut values_result);
        }

        println!("Keys: {:?}, Values: {:?}", keys_result, values_result);
        println!("{:?}", values_result[0].parse::<f64>().unwrap());

        let expected_keys_result = ["some_price"];
        let expected_values_result = ["200.44"];
        assert_eq!(keys_result, expected_keys_result);
        assert_eq!(values_result, expected_values_result);
    }

    #[test]
    fn parse_sequance_value_from_yaml() {
        let mut keys_result: Vec<String> = Vec::new();
        let mut values_result: Vec<String> = Vec::new();

        let yaml_input: serde_yaml::Value = serde_yaml::from_str(
            r#"
  imagePullSecrets:
    - name: docker-pull-secret
"#,
        )
        .expect("Failed to parse YAML");

        for (key, value) in yaml_input.as_mapping().unwrap() {
            let new_key = key.as_str().unwrap();
            handle_root_sequence_value_type_recursively(
                new_key,
                value,
                &mut keys_result,
                &mut values_result,
            );
        }

        println!("Keys: {:?}, Values: {:?}", keys_result, values_result);

        let expected_keys_result = ["imagePullSecrets.name"];
        let expected_values_result = ["docker-pull-secret"];
        assert_eq!(keys_result, expected_keys_result);
        assert_eq!(values_result, expected_values_result);
    }

    #[test]
    fn parse_mapping_value_from_yaml() {
        let mut keys_result: Vec<String> = Vec::new();
        let mut values_result: Vec<String> = Vec::new();

        let yaml_input: serde_yaml::Value = serde_yaml::from_str(
            r#"
cleanup_job:
  imagePullSecrets:
    - name: docker-pull-secret
"#,
        )
        .expect("Failed to parse YAML");

        for (key, value) in yaml_input.as_mapping().unwrap() {
            let new_key = key.as_str().unwrap();
            handle_root_mapping_value_type_recursively(
                new_key,
                value,
                &mut keys_result,
                &mut values_result,
            );
        }

        println!("Keys: {:?}, Values: {:?}", keys_result, values_result);

        let expected_keys_result = ["cleanup_job.imagePullSecrets.name"];
        let expected_values_result = ["docker-pull-secret"];
        assert_eq!(keys_result, expected_keys_result);
        assert_eq!(values_result, expected_values_result);
    }
}
