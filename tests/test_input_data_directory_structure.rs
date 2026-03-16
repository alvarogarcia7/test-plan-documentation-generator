use std::fs;
use std::path::Path;

const INPUT_DATA_DIR: &str = "data/input_data";

#[test]
fn test_input_data_directory_exists() {
    let input_data_path = Path::new(INPUT_DATA_DIR);
    assert!(
        input_data_path.exists(),
        "data/input_data directory must exist"
    );
    assert!(
        input_data_path.is_dir(),
        "data/input_data must be a directory"
    );
}

#[test]
fn test_required_subdirectories_exist() {
    let required_dirs = vec![
        "container",
        "test_case",
        "test_results",
        "verification_methods",
    ];

    for dir_name in required_dirs {
        let dir_path = Path::new(INPUT_DATA_DIR).join(dir_name);
        assert!(
            dir_path.exists(),
            "{} directory must exist in data/input_data/",
            dir_name
        );
        assert!(dir_path.is_dir(), "{} must be a directory", dir_name);
    }
}

#[test]
fn test_container_directory_structure() {
    let container_dir = Path::new(INPUT_DATA_DIR).join("container");

    let schema_file = container_dir.join("schema.json");
    assert!(schema_file.exists(), "container/schema.json must exist");
    assert!(
        schema_file.is_file(),
        "container/schema.json must be a file"
    );

    let template_file = container_dir.join("template.j2");
    assert!(template_file.exists(), "container/template.j2 must exist");
    assert!(
        template_file.is_file(),
        "container/template.j2 must be a file"
    );

    let data_file = container_dir.join("data.yml");
    assert!(data_file.exists(), "container/data.yml must exist");
    assert!(data_file.is_file(), "container/data.yml must be a file");
}

#[test]
fn test_test_case_directory_has_yaml_files() {
    let test_case_dir = Path::new(INPUT_DATA_DIR).join("test_case");

    let entries = fs::read_dir(&test_case_dir).expect("Failed to read test_case directory");

    let yaml_files: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().is_file() && {
                let path = entry.path();
                path.extension()
                    .map(|ext| ext == "yml" || ext == "yaml")
                    .unwrap_or(false)
            }
        })
        .collect();

    assert!(
        !yaml_files.is_empty(),
        "test_case directory must contain at least one YAML file (.yml or .yaml)"
    );
}

#[test]
fn test_test_results_directory_structure() {
    let test_results_dir = Path::new(INPUT_DATA_DIR).join("test_results");

    let schema_file = test_results_dir.join("container_schema.json");
    assert!(
        schema_file.exists(),
        "test_results/container_schema.json must exist"
    );
    assert!(
        schema_file.is_file(),
        "test_results/container_schema.json must be a file"
    );

    let data_file = test_results_dir.join("container_data.yml");
    assert!(
        data_file.exists(),
        "test_results/container_data.yml must exist"
    );
    assert!(
        data_file.is_file(),
        "test_results/container_data.yml must be a file"
    );
}

#[test]
fn test_verification_methods_directory_structure() {
    let vm_dir = Path::new(INPUT_DATA_DIR).join("verification_methods");

    let required_vm_types = vec!["test", "analysis", "inspection", "demonstration", "result"];

    for vm_type in required_vm_types {
        let vm_type_dir = vm_dir.join(vm_type);
        assert!(
            vm_type_dir.exists(),
            "verification_methods/{} directory must exist",
            vm_type
        );
        assert!(
            vm_type_dir.is_dir(),
            "verification_methods/{} must be a directory",
            vm_type
        );

        let schema_file = vm_type_dir.join("schema.json");
        assert!(
            schema_file.exists(),
            "verification_methods/{}/schema.json must exist",
            vm_type
        );
        assert!(
            schema_file.is_file(),
            "verification_methods/{}/schema.json must be a file",
            vm_type
        );

        if vm_type != "result" {
            let template_file = vm_type_dir.join("template.j2");
            assert!(
                template_file.exists(),
                "verification_methods/{}/template.j2 must exist",
                vm_type
            );
            assert!(
                template_file.is_file(),
                "verification_methods/{}/template.j2 must be a file",
                vm_type
            );
        }
    }
}

#[test]
fn test_container_schema_json_is_valid() {
    let schema_path = Path::new(INPUT_DATA_DIR).join("container/schema.json");
    let schema_content =
        fs::read_to_string(&schema_path).expect("Failed to read container/schema.json");

    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&schema_content);
    assert!(
        parsed.is_ok(),
        "container/schema.json must be valid JSON: {:?}",
        parsed.err()
    );

    let schema_value = parsed.unwrap();
    assert!(
        schema_value.get("$schema").is_some(),
        "container/schema.json must have a $schema field"
    );

    let schema_uri = schema_value.get("$schema").and_then(|s| s.as_str());
    assert!(schema_uri.is_some(), "$schema field must be a string");

    let uri = schema_uri.unwrap();
    assert!(
        uri.contains("json-schema.org/draft-04") || uri.contains("json-schema.org/draft-07"),
        "Schema must conform to JSON Schema draft-04 or draft-07, found: {}",
        uri
    );
}

#[test]
fn test_test_results_container_schema_json_is_valid() {
    let schema_path = Path::new(INPUT_DATA_DIR).join("test_results/container_schema.json");
    let schema_content = fs::read_to_string(&schema_path)
        .expect("Failed to read test_results/container_schema.json");

    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&schema_content);
    assert!(
        parsed.is_ok(),
        "test_results/container_schema.json must be valid JSON: {:?}",
        parsed.err()
    );

    let schema_value = parsed.unwrap();
    assert!(
        schema_value.get("$schema").is_some(),
        "test_results/container_schema.json must have a $schema field"
    );

    let schema_uri = schema_value.get("$schema").and_then(|s| s.as_str());
    assert!(schema_uri.is_some(), "$schema field must be a string");

    let uri = schema_uri.unwrap();
    assert!(
        uri.contains("json-schema.org/draft-04") || uri.contains("json-schema.org/draft-07"),
        "Schema must conform to JSON Schema draft-04 or draft-07, found: {}",
        uri
    );
}

#[test]
fn test_verification_methods_schemas_are_valid() {
    let vm_dir = Path::new(INPUT_DATA_DIR).join("verification_methods");
    let vm_types = vec!["test", "analysis", "inspection", "demonstration", "result"];

    for vm_type in vm_types {
        let schema_path = vm_dir.join(format!("{}/schema.json", vm_type));
        let schema_content = fs::read_to_string(&schema_path).unwrap_or_else(|_| {
            panic!(
                "Failed to read verification_methods/{}/schema.json",
                vm_type
            )
        });

        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&schema_content);
        assert!(
            parsed.is_ok(),
            "verification_methods/{}/schema.json must be valid JSON: {:?}",
            vm_type,
            parsed.err()
        );

        let schema_value = parsed.unwrap();
        assert!(
            schema_value.get("$schema").is_some(),
            "verification_methods/{}/schema.json must have a $schema field",
            vm_type
        );

        let schema_uri = schema_value.get("$schema").and_then(|s| s.as_str());
        assert!(
            schema_uri.is_some(),
            "verification_methods/{}/schema.json $schema field must be a string",
            vm_type
        );

        let uri = schema_uri.unwrap();
        assert!(
			uri.contains("json-schema.org/draft-04") || uri.contains("json-schema.org/draft-07"),
			"verification_methods/{}/schema.json must conform to JSON Schema draft-04 or draft-07, found: {}",
			vm_type,
			uri
		);
    }
}

#[test]
fn test_container_data_yaml_is_valid() {
    let data_path = Path::new(INPUT_DATA_DIR).join("container/data.yml");
    let data_content = fs::read_to_string(&data_path).expect("Failed to read container/data.yml");

    let parsed: Result<serde_yaml::Value, _> = serde_yaml::from_str(&data_content);
    assert!(
        parsed.is_ok(),
        "container/data.yml must be valid YAML: {:?}",
        parsed.err()
    );
}

#[test]
fn test_test_results_container_data_yaml_is_valid() {
    let data_path = Path::new(INPUT_DATA_DIR).join("test_results/container_data.yml");
    let data_content =
        fs::read_to_string(&data_path).expect("Failed to read test_results/container_data.yml");

    let parsed: Result<serde_yaml::Value, _> = serde_yaml::from_str(&data_content);
    assert!(
        parsed.is_ok(),
        "test_results/container_data.yml must be valid YAML: {:?}",
        parsed.err()
    );
}

#[test]
fn test_test_case_yaml_files_are_valid() {
    let test_case_dir = Path::new(INPUT_DATA_DIR).join("test_case");

    let entries = fs::read_dir(&test_case_dir).expect("Failed to read test_case directory");

    let yaml_files: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let path = entry.path();
            path.is_file()
                && path
                    .extension()
                    .map(|ext| ext == "yml" || ext == "yaml")
                    .unwrap_or(false)
        })
        .collect();

    assert!(
        !yaml_files.is_empty(),
        "test_case directory must contain at least one YAML file"
    );

    for yaml_file in yaml_files {
        let path = yaml_file.path();
        let content =
            fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read {:?}", path));

        let parsed: Result<serde_yaml::Value, _> = serde_yaml::from_str(&content);
        assert!(
            parsed.is_ok(),
            "{:?} must be valid YAML: {:?}",
            path,
            parsed.err()
        );
    }
}

#[test]
fn test_container_template_has_valid_tera_syntax() {
    let template_path = Path::new(INPUT_DATA_DIR).join("container/template.j2");
    let template_content =
        fs::read_to_string(&template_path).expect("Failed to read container/template.j2");

    let mut tera = tera::Tera::default();
    let result = tera.add_raw_template("container_template", &template_content);

    assert!(
        result.is_ok(),
        "container/template.j2 must have valid Tera/Jinja2 syntax: {:?}",
        result.err()
    );
}

#[test]
fn test_verification_methods_templates_have_valid_tera_syntax() {
    let vm_dir = Path::new(INPUT_DATA_DIR).join("verification_methods");
    let vm_types = vec!["test", "analysis", "inspection", "demonstration"];

    for vm_type in vm_types {
        let template_path = vm_dir.join(format!("{}/template.j2", vm_type));
        let template_content = fs::read_to_string(&template_path).unwrap_or_else(|_| {
            panic!(
                "Failed to read verification_methods/{}/template.j2",
                vm_type
            )
        });

        let mut tera = tera::Tera::default();
        let result = tera.add_raw_template(&format!("{}_template", vm_type), &template_content);

        assert!(
            result.is_ok(),
            "verification_methods/{}/template.j2 must have valid Tera/Jinja2 syntax: {:?}",
            vm_type,
            result.err()
        );
    }
}

#[test]
fn test_schema_json_files_validate_with_jsonschema() {
    let schema_files = vec![
        "container/schema.json",
        "test_results/container_schema.json",
        "verification_methods/test/schema.json",
        "verification_methods/analysis/schema.json",
        "verification_methods/inspection/schema.json",
        "verification_methods/demonstration/schema.json",
        "verification_methods/result/schema.json",
    ];

    for schema_file in schema_files {
        let schema_path = Path::new(INPUT_DATA_DIR).join(schema_file);
        let schema_content = fs::read_to_string(&schema_path)
            .unwrap_or_else(|_| panic!("Failed to read {}", schema_file));

        let schema_json: serde_json::Value = serde_json::from_str(&schema_content)
            .unwrap_or_else(|_| panic!("{} must be valid JSON", schema_file));

        let compiled = jsonschema::JSONSchema::compile(&schema_json);
        assert!(
            compiled.is_ok(),
            "{} must be a valid JSON Schema that can be compiled: {:?}",
            schema_file,
            compiled.err()
        );
    }
}

#[test]
fn test_all_required_files_are_present() {
    let required_files = vec![
        "container/schema.json",
        "container/template.j2",
        "container/data.yml",
        "test_results/container_schema.json",
        "test_results/container_data.yml",
        "verification_methods/test/schema.json",
        "verification_methods/test/template.j2",
        "verification_methods/analysis/schema.json",
        "verification_methods/analysis/template.j2",
        "verification_methods/inspection/schema.json",
        "verification_methods/inspection/template.j2",
        "verification_methods/demonstration/schema.json",
        "verification_methods/demonstration/template.j2",
        "verification_methods/result/schema.json",
    ];

    for file_path in required_files {
        let full_path = Path::new(INPUT_DATA_DIR).join(file_path);
        assert!(full_path.exists(), "Required file {} must exist", file_path);
        assert!(full_path.is_file(), "{} must be a file", file_path);
    }
}

#[test]
fn test_no_empty_schema_files() {
    let schema_files = vec![
        "container/schema.json",
        "test_results/container_schema.json",
        "verification_methods/test/schema.json",
        "verification_methods/analysis/schema.json",
        "verification_methods/inspection/schema.json",
        "verification_methods/demonstration/schema.json",
        "verification_methods/result/schema.json",
    ];

    for schema_file in schema_files {
        let schema_path = Path::new(INPUT_DATA_DIR).join(schema_file);
        let schema_content = fs::read_to_string(&schema_path)
            .unwrap_or_else(|_| panic!("Failed to read {}", schema_file));

        assert!(
            !schema_content.trim().is_empty(),
            "{} must not be empty",
            schema_file
        );

        let schema_json: serde_json::Value = serde_json::from_str(&schema_content)
            .unwrap_or_else(|_| panic!("{} must be valid JSON", schema_file));

        assert!(
            schema_json != serde_json::json!({}),
            "{} must not be an empty JSON object",
            schema_file
        );
    }
}

#[test]
fn test_no_empty_template_files() {
    let template_files = vec![
        "container/template.j2",
        "verification_methods/test/template.j2",
        "verification_methods/analysis/template.j2",
        "verification_methods/inspection/template.j2",
        "verification_methods/demonstration/template.j2",
    ];

    for template_file in template_files {
        let template_path = Path::new(INPUT_DATA_DIR).join(template_file);
        let template_content = fs::read_to_string(&template_path)
            .unwrap_or_else(|_| panic!("Failed to read {}", template_file));

        assert!(
            !template_content.trim().is_empty(),
            "{} must not be empty or contain only whitespace",
            template_file
        );
    }
}

#[test]
fn test_yaml_data_files_are_not_empty() {
    let yaml_files = vec!["container/data.yml", "test_results/container_data.yml"];

    for yaml_file in yaml_files {
        let yaml_path = Path::new(INPUT_DATA_DIR).join(yaml_file);
        let yaml_content = fs::read_to_string(&yaml_path)
            .unwrap_or_else(|_| panic!("Failed to read {}", yaml_file));

        let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml_content)
            .unwrap_or_else(|_| panic!("{} must be valid YAML", yaml_file));

        assert!(
            parsed != serde_yaml::Value::Null,
            "{} must not be null or empty",
            yaml_file
        );
    }
}
