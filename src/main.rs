use anyhow::Result;
use clap::Parser;
use jsonschema::JSONSchema;
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::os::unix::io::FromRawFd;
use std::path::PathBuf;
use std::process::exit;
use tera::Context;
use tera::Tera;

#[derive(Parser, Debug)]
#[command(name = "test-plan-doc-gen")]
#[command(about = "A CLI tool to render templates for test plan documentation", long_about = None)]
struct Args {
    /// Output file path
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Container schema file
    #[arg(long = "container", value_names = ["CONTAINER_SCHEMA", "CONTAINER_TEMPLATE", "CONTAINER_FILE"], num_args = 3, required = true)]
    container: Vec<PathBuf>,

    /// Test case verification methods directory and files
    #[arg(long = "test-case", value_names = ["VERIFICATION_METHODS_DIR", "TEST_CASE_FILE", "REST_TEST_CASE_FILES"], num_args = 2.., required = true)]
    test_case: Vec<PathBuf>,

    /// Output format (markdown or asciidoc)
    #[arg(long = "format", default_value = "markdown", value_parser = ["markdown", "asciidoc"])]
    format: String,
}

#[cfg(test)]
fn render_template(template_str: &str, context: &tera::Context) -> Result<String> {
    let mut tera = Tera::default();
    tera.add_raw_template("template", template_str)?;
    let rendered = tera.render("template", context)?;
    Ok(rendered)
}

// Helper to get a File for fd 3 (do not close on drop)
fn log_file() -> File {
    // SAFETY: We must not close fd 3, so we use into_raw_fd to avoid double-close.
    unsafe { File::from_raw_fd(3) }
}

// Logging macro
macro_rules! log_fd3 {
    ($($arg:tt)*) => {{
        use std::io::Write;
        let mut file = log_file();
        let _ = writeln!(file, $($arg)*);
        std::mem::forget(file); // Prevent closing fd 3
    }};
}

fn get_template_suffix(format: &str) -> &str {
    match format {
        "markdown" => ".j2",
        "asciidoc" => "_asciidoc.adoc",
        _ => ".j2",
    }
}

fn main() -> Result<()> {
    log_fd3!("Starting test-plan-doc-gen");
    tracing_subscriber::fmt::init();

    // Log the raw arguments before parsing
    let raw_args: Vec<String> = std::env::args().collect();
    log_fd3!("Raw arguments: {:?}", raw_args);

    let args = Args::parse();
    log_fd3!("Parsed arguments: {:?}", args);

    // Parse container arguments
    let container_schema = &args.container[0];
    let container_template = &args.container[1];
    let container_file = &args.container[2];

    // Parse test case arguments - first is verification methods directory
    let verification_methods_dir = &args.test_case[0];
    let test_case_files = if args.test_case.len() > 1 {
        &args.test_case[1..]
    } else {
        usage(1)
    };

    // Verify verification methods directory exists
    if !verification_methods_dir.exists() || !verification_methods_dir.is_dir() {
        let message = format!(
            "Error: Verification methods directory does not exist or is not a directory: {}",
            verification_methods_dir.display()
        );
        log_fd3!("{}", message);
        eprintln!("{}", message);
        exit(2);
    }

    // Verify that all received files exist
    let mut missing_files = Vec::new();
    let all_files = [
        vec![container_schema, container_template, container_file],
        test_case_files.iter().collect::<Vec<_>>(),
    ]
    .concat();
    log_fd3!("Checking file existence...");
    for file in &all_files {
        if !file.exists() {
            missing_files.push(file.display().to_string());
        }
    }
    if !missing_files.is_empty() {
        let message = "Error: The following files do not exist:\n{}";
        let missing_files_as_str = missing_files.join("\n");
        log_fd3!("{} {}", message, missing_files_as_str);
        eprintln!("{} {}", message, missing_files_as_str);
        exit(2);
    }
    log_fd3!("All files exist, proceeding.");

    // First pass: read all YAML files to get their types and validate they can be loaded
    let mut file_types: HashMap<PathBuf, String> = HashMap::new();
    for file in test_case_files {
        log_fd3!("Reading type from file: {}", file.display());
        let content = fs::read_to_string(file)?;
        let yaml_val: YamlValue = serde_yaml::from_str(&content)?;

        // Extract the type field
        let type_value = if let YamlValue::Mapping(ref map) = yaml_val {
            map.iter()
                .find(|(k, _)| k.as_str() == Some("type"))
                .and_then(|(_, v)| v.as_str())
        } else {
            None
        };

        let type_str = type_value.ok_or_else(|| {
            anyhow::anyhow!("File {} does not have a 'type' field", file.display())
        })?;

        log_fd3!("File {} has type: {}", file.display(), type_str);
        file_types.insert(file.clone(), type_str.to_string());
    }

    // Build a map of type -> (schema_path, template_path)
    let template_suffix = get_template_suffix(&args.format);
    let mut type_resources: HashMap<String, (PathBuf, PathBuf)> = HashMap::new();
    for type_name in file_types.values() {
        if !type_resources.contains_key(type_name) {
            let type_dir = verification_methods_dir.join(type_name);
            let schema_path = type_dir.join("schema.json");
            let template_filename = format!("template{}", template_suffix);
            let template_path = type_dir.join(&template_filename);

            // Verify these files exist
            if !schema_path.exists() {
                let message = format!(
                    "Error: Schema file does not exist: {}",
                    schema_path.display()
                );
                log_fd3!("{}", message);
                eprintln!("{}", message);
                exit(2);
            }
            if !template_path.exists() {
                let message = format!(
                    "Error: Template file does not exist: {}",
                    template_path.display()
                );
                log_fd3!("{}", message);
                eprintln!("{}", message);
                exit(2);
            }

            log_fd3!(
                "Type '{}' uses schema: {} and template: {}",
                type_name,
                schema_path.display(),
                template_path.display()
            );
            type_resources.insert(type_name.clone(), (schema_path, template_path));
        }
    }

    // --- Validate and render all test-case files ---
    // Group files by type to efficiently load templates
    // First sort file_types by key to ensure deterministic iteration order
    let mut sorted_file_types: Vec<_> = file_types.iter().collect();
    sorted_file_types.sort_by_key(|(file, _)| *file);

    let mut files_by_type: HashMap<String, Vec<&PathBuf>> = HashMap::new();
    for (file, type_name) in sorted_file_types {
        files_by_type
            .entry(type_name.clone())
            .or_default()
            .push(file);
    }

    // Sort files within each type to ensure deterministic output order
    for files in files_by_type.values_mut() {
        files.sort();
    }

    let mut concatenated = String::new();
    let mut first = true;

    // Sort type names to ensure deterministic output order
    let mut sorted_types: Vec<_> = files_by_type.keys().collect();
    sorted_types.sort();

    for type_name in sorted_types {
        let files = &files_by_type[type_name];
        let (schema_path, template_path) = &type_resources[type_name];

        log_fd3!("Processing files of type '{}'", type_name);

        // Load template once per type
        let template_str = fs::read_to_string(template_path)?;
        let mut tera = Tera::default();
        tera.add_raw_template("tc_template", &template_str)?;

        // Process each file of this type
        for file in files {
            log_fd3!("Validating test-case file: {}", file.display());
            log_fd3!("\tAgainst schema: {}", schema_path.display());

            let content = fs::read_to_string(file)?;
            let yaml_val: YamlValue =
                serde_yaml::from_str(&content).unwrap_or_else(|_| YamlValue::Null);
            let json_value: JsonValue = serde_json::from_str(&serde_json::to_string(&yaml_val)?)?;

            let validation_result: Result<(), Vec<String>> =
                validate_json_schema(schema_path, &json_value);
            match validation_result {
                Ok(_) => {
                    log_fd3!("\tValidation successful.");
                }
                Err(errors) => {
                    let message = format!(
                        "Error: Validation failed for file: {}\nAgainst schema: {}",
                        file.display(),
                        schema_path.display()
                    );
                    log_fd3!("{}", message);
                    eprintln!("{}", message);
                    for error in &errors {
                        let error_msg = format!("  - {}", error);
                        log_fd3!("{}", error_msg);
                        eprintln!("{}", error_msg);
                    }
                    exit(3);
                }
            }

            // Render the file
            log_fd3!("Loading test-case data from: {}", file.display());

            // Build Tera context from YAML mapping (if applicable)
            let mut tc_context = Context::new();
            if let YamlValue::Mapping(map) = &yaml_val {
                for (key, value) in map {
                    if let Some(key_str) = key.as_str() {
                        let json_str = serde_json::to_string(value)?;
                        let json_value: JsonValue = serde_json::from_str(&json_str)?;
                        tc_context.insert(key_str, &json_value);
                    }
                }
            }

            // Also insert the whole data under `data`
            let json_value_full: JsonValue =
                serde_json::from_str(&serde_json::to_string(&yaml_val)?)?;
            tc_context.insert("data", &json_value_full);

            // Render
            log_fd3!("Rendering test-case template for: {}", file.display());
            let rendered = tera.render("tc_template", &tc_context)?;
            if !first {
                concatenated.push_str("\n\n");
            }
            first = false;
            concatenated.push_str(&rendered);
        }
    }
    log_fd3!("Rendering test-case files completed");

    // Create a unique temporary directory under the OS temp dir and write output.md
    let unique = format!(
        "test-plan-doc-gen-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_nanos()
    );
    let tmp_dir = std::env::temp_dir().join(&unique);
    log_fd3!("Created temporary folder: {:?}", unique);
    fs::create_dir_all(&tmp_dir)?;
    let output_md_path = tmp_dir.join("output.md");
    fs::write(&output_md_path, &concatenated)?;
    log_fd3!(
        "Rendered test-case markdown to: {}",
        output_md_path.display()
    );

    // Make the rendered test-cases available to container template via context
    // under `test_cases_md` (string) and `test_cases_path` (path string)
    // These will be inserted into the container context later.

    // Load YAML data from the container file
    log_fd3!("Loading container data from: {}", container_file.display());
    let yaml_str = fs::read_to_string(container_file)?;
    let yaml_data: YamlValue = serde_yaml::from_str(&yaml_str)?;

    // Convert YAML data to JSON value for validation and Tera
    let json_value: JsonValue = serde_json::from_str(&serde_json::to_string(&yaml_data)?)?;

    // Validate against container schema
    log_fd3!("Validating container file: {}", container_file.display());
    log_fd3!("\tAgainst schema: {}", container_schema.display());
    let validation_result: Result<(), Vec<String>> =
        validate_json_schema(container_schema, &json_value);
    match validation_result {
        Ok(_) => {
            log_fd3!("\tValidation successful.");
        }
        Err(errors) => {
            let message = format!(
                "Error: Validation failed for file: {}\nAgainst schema: {}",
                container_file.display(),
                container_schema.display()
            );
            log_fd3!("{}", message);
            eprintln!("{}", message);
            for error in &errors {
                let error_msg = format!("  - {}", error);
                log_fd3!("{}", error_msg);
                eprintln!("{}", error_msg);
            }
            exit(3);
        }
    }

    // Validate individual test_results entries against verification_schema.json
    if let Some(test_results) = json_value.get("test_results") {
        if let Some(test_results_array) = test_results.as_array() {
            log_fd3!(
                "Validating {} test result entries in container file",
                test_results_array.len()
            );

            // Find verification_schema.json in the schemas directory
            let verification_schema_path =
                PathBuf::from("data/dataset_4_GSMA/schemas/verification_schema.json");
            if verification_schema_path.exists() {
                log_fd3!(
                    "\tUsing verification schema: {}",
                    verification_schema_path.display()
                );

                for (index, test_result) in test_results_array.iter().enumerate() {
                    log_fd3!("\tValidating test result entry #{}", index + 1);
                    let validation_result =
                        validate_json_schema(&verification_schema_path, test_result);
                    match validation_result {
                        Ok(_) => {
                            log_fd3!("\t\tValidation successful for entry #{}", index + 1);
                        }
                        Err(errors) => {
                            let default_id = format!("entry #{}", index + 1);
                            let test_case_id = test_result
                                .get("test_case_id")
                                .and_then(|v| v.as_str())
                                .unwrap_or(&default_id);
                            let message = format!(
                                "Error: Validation failed for test result entry: {}\nIn file: {}\nAgainst schema: {}",
                                test_case_id,
                                container_file.display(),
                                verification_schema_path.display()
                            );
                            log_fd3!("{}", message);
                            eprintln!("{}", message);
                            for error in &errors {
                                let error_msg = format!("  - {}", error);
                                log_fd3!("{}", error_msg);
                                eprintln!("{}", error_msg);
                            }
                            exit(3);
                        }
                    }
                }
            } else {
                log_fd3!("\tWarning: verification_schema.json not found at {}, skipping test result validation", verification_schema_path.display());
            }
        }
    }

    // Convert YAML data to Tera context
    // We convert through JSON to ensure proper serialization for Tera
    let mut context = Context::new();
    // If the top-level YAML was a mapping, insert its keys into context
    if let YamlValue::Mapping(map) = yaml_data {
        for (key, value) in map {
            if let Some(key_str) = key.as_str() {
                log_fd3!("\tFound key: {}", key_str);
                // Convert YAML value to JSON value for proper Tera serialization
                let json_str = serde_json::to_string(&value)?;
                let json_value: JsonValue = serde_json::from_str(&json_str)?;
                context.insert(key_str, &json_value);
            }
        }
    }

    // Inject the rendered test-case markdown and its path into the container context
    context.insert("test_cases_md", &concatenated);
    context.insert(
        "test_cases_path",
        &output_md_path.to_string_lossy().to_string(),
    );

    // Load and render requirement_aggregation_template.adoc
    let req_agg_template_path =
        verification_methods_dir.join("requirement_aggregation_template.adoc");
    if req_agg_template_path.exists() {
        log_fd3!(
            "Loading requirement aggregation template from: {}",
            req_agg_template_path.display()
        );
        let req_agg_template_str = fs::read_to_string(&req_agg_template_path)?;
        let mut req_tera = Tera::default();
        req_tera.add_raw_template("req_agg_template", &req_agg_template_str)?;

        log_fd3!("Rendering requirement aggregation template...");
        match req_tera.render("req_agg_template", &context) {
            Ok(requirements_summary) => {
                context.insert("requirements_summary_adoc", &requirements_summary);
                log_fd3!("Requirements summary rendered and added to context");
            }
            Err(e) => {
                log_fd3!(
                    "Warning: Failed to render requirement aggregation template: {}",
                    e
                );
            }
        }
    } else {
        log_fd3!(
            "Warning: requirement_aggregation_template.adoc not found at {}",
            req_agg_template_path.display()
        );
    }

    // Read the template file
    let template_str = fs::read_to_string(container_template)?;
    let mut tera = Tera::default();
    tera.add_raw_template("template", &template_str)?;

    // Render the template
    let rendered = tera.render("template", &context)?;

    // Write the output
    log_fd3!("Rendering container template...");
    if let Some(output_path) = args.output {
        log_fd3!("Rendering container to file...");
        fs::write(&output_path, &rendered)?;
        eprintln!(
            "Template rendered successfully to {}",
            output_path.display()
        );
        log_fd3!("Rendered output, writing to {:?}", output_path);
    } else {
        log_fd3!("Rendering container to console. You can redirect the file descriptor 3>log_fd3.txt to capture logs.");
        println!("{}", rendered);
    }

    Ok(())
}

fn validate_json_schema(
    schema_path: &PathBuf,
    payload: &serde_json::Value,
) -> Result<(), Vec<String>> {
    let schema_str = fs::read_to_string(schema_path);
    let schema_str = match schema_str {
        Ok(s) => s,
        Err(e) => {
            let error_msg = format!(
                "Failed to read schema file {}: {}",
                schema_path.display(),
                e
            );
            log_fd3!("{}", error_msg);
            return Err(vec![error_msg]);
        }
    };
    let schema_str_2 = schema_str;
    let schema_json = serde_json::from_str(&schema_str_2);
    let schema_json = match schema_json {
        Ok(s) => s,
        Err(e) => {
            let error_msg = format!(
                "Failed to parse schema file {}: {}",
                schema_path.display(),
                e
            );
            log_fd3!("{}", error_msg);
            return Err(vec![error_msg]);
        }
    };
    // JSONSchema::compile requires the schema to live for 'static because the
    // compiled schema holds references into it. Allocate the schema on the heap
    // and leak it to get a &'static reference. This is acceptable for a short-
    // lived CLI process.
    let schema_box = Box::new(schema_json);
    let schema_static: &'static JsonValue = Box::leak(schema_box);
    let compiled = JSONSchema::compile(schema_static);
    let compiled2 = match compiled {
        Ok(c) => c,
        Err(e) => {
            let error_msg = format!(
                "Failed to compile schema from file {}: {}",
                schema_path.display(),
                e
            );
            log_fd3!("{}", error_msg);
            return Err(vec![error_msg]);
        }
    };
    log_fd3!("\tValidating payload against schema...");
    let validation_result = compiled2.validate(payload);
    match validation_result {
        Ok(_) => {
            log_fd3!("\tSchema validation: VALID");
            Ok(())
        }
        Err(errors) => {
            log_fd3!("\tSchema validation: INVALID");
            let error_messages: Vec<String> = errors.into_iter().map(|e| e.to_string()).collect();
            Err(error_messages)
        }
    }
}

fn usage(ret_code: i32) -> ! {
    log_fd3!("Wrong usage? Returning status code {}", ret_code);
    exit(ret_code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_render_simple_template() {
        let template = "Hello, {{ name }}!";
        let mut context = tera::Context::new();
        context.insert("name", "World");

        let result = render_template(template, &context).expect("Failed to render template");
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_render_template_with_no_variables() {
        let template = "Static content";
        let context = tera::Context::new();

        let result = render_template(template, &context).expect("Failed to render template");
        assert_eq!(result, "Static content");
    }

    #[test]
    fn test_render_template_with_loop() {
        let template = "{% for item in items %}{{ item }}\n{% endfor %}";
        let mut context = tera::Context::new();
        context.insert("items", &vec!["a", "b", "c"]);

        let result = render_template(template, &context).expect("Failed to render template");
        assert_eq!(result, "a\nb\nc\n");
    }

    #[test]
    fn test_render_template_with_condition() {
        let template = "{% if show %}Visible{% endif %}";
        let mut context = tera::Context::new();
        context.insert("show", &true);

        let result = render_template(template, &context).expect("Failed to render template");
        assert_eq!(result, "Visible");
    }

    #[test]
    fn test_invalid_template_syntax() {
        let template = "{{ unclosed";
        let context = tera::Context::new();

        let result = render_template(template, &context);
        assert!(result.is_err(), "Should fail for invalid template syntax");
    }

    // CLI Argument Parser Tests

    #[test]
    fn test_parse_minimal_valid_args() {
        let args = Args::try_parse_from([
            "test-plan-doc-gen",
            "--container",
            "schema.json",
            "template.tera",
            "container.json",
            "--test-case",
            "verification_methods",
            "test1.json",
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.container.len(), 3);
        assert_eq!(args.test_case.len(), 2);
        assert!(args.output.is_none());
    }

    #[test]
    fn test_parse_with_output_file() {
        let args = Args::try_parse_from([
            "test-plan-doc-gen",
            "-o",
            "output.md",
            "--container",
            "schema.json",
            "template.tera",
            "container.json",
            "--test-case",
            "verification_methods",
            "test1.json",
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert!(args.output.is_some());
        assert_eq!(args.output.unwrap(), PathBuf::from("output.md"));
    }

    #[test]
    fn test_parse_with_long_output_flag() {
        let args = Args::try_parse_from([
            "test-plan-doc-gen",
            "--output",
            "result.html",
            "--container",
            "schema.json",
            "template.tera",
            "container.json",
            "--test-case",
            "verification_methods",
            "test1.json",
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert!(args.output.is_some());
        assert_eq!(args.output.unwrap(), PathBuf::from("result.html"));
    }

    #[test]
    fn test_parse_with_multiple_test_case_files() {
        let args = Args::try_parse_from([
            "test-plan-doc-gen",
            "--container",
            "schema.json",
            "template.tera",
            "container.json",
            "--test-case",
            "verification_methods",
            "test1.json",
            "test2.json",
            "test3.json",
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.test_case.len(), 4);
    }

    #[test]
    fn test_parse_container_values() {
        let args = Args::try_parse_from([
            "test-plan-doc-gen",
            "--container",
            "container_schema.json",
            "container_template.tera",
            "container_data.json",
            "--test-case",
            "verification_methods",
            "test1.json",
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.container[0], PathBuf::from("container_schema.json"));
        assert_eq!(args.container[1], PathBuf::from("container_template.tera"));
        assert_eq!(args.container[2], PathBuf::from("container_data.json"));
    }

    #[test]
    fn test_parse_test_case_values() {
        let args = Args::try_parse_from([
            "test-plan-doc-gen",
            "--container",
            "schema.json",
            "template.tera",
            "container.json",
            "--test-case",
            "verification_methods",
            "test1.json",
            "test2.json",
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.test_case[0], PathBuf::from("verification_methods"));
        assert_eq!(args.test_case[1], PathBuf::from("test1.json"));
        assert_eq!(args.test_case[2], PathBuf::from("test2.json"));
    }

    #[test]
    fn test_parse_missing_container_arg() {
        let args = Args::try_parse_from([
            "test-plan-doc-gen",
            "--test-case",
            "verification_methods",
            "test1.json",
        ]);

        assert!(args.is_err());
    }

    #[test]
    fn test_parse_missing_test_case_arg() {
        let args = Args::try_parse_from([
            "test-plan-doc-gen",
            "--container",
            "schema.json",
            "template.tera",
            "container.json",
        ]);

        assert!(args.is_err());
    }

    #[test]
    fn test_parse_container_with_insufficient_args() {
        let args = Args::try_parse_from([
            "test-plan-doc-gen",
            "--container",
            "schema.json",
            "template.tera",
            "--test-case",
            "verification_methods",
            "test1.json",
        ]);

        assert!(args.is_err());
    }

    #[test]
    fn test_parse_test_case_with_insufficient_args() {
        let args = Args::try_parse_from([
            "test-plan-doc-gen",
            "--container",
            "schema.json",
            "template.tera",
            "container.json",
            "--test-case",
            "verification_methods",
        ]);

        assert!(args.is_err());
    }

    #[test]
    fn test_parse_container_with_too_many_args() {
        let args = Args::try_parse_from([
            "test-plan-doc-gen",
            "--container",
            "schema.json",
            "template.tera",
            "container.json",
            "extra.json",
            "--test-case",
            "verification_methods",
            "test1.json",
        ]);

        assert!(args.is_err());
    }

    #[test]
    fn test_parse_args_order_independence() {
        let args = Args::try_parse_from([
            "test-plan-doc-gen",
            "--test-case",
            "verification_methods",
            "test1.json",
            "-o",
            "output.md",
            "--container",
            "schema.json",
            "template.tera",
            "container.json",
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert!(args.output.is_some());
        assert_eq!(args.container.len(), 3);
        assert_eq!(args.test_case.len(), 2);
    }

    #[test]
    fn test_command_help_generation() {
        let mut cmd = Args::command();
        let help = cmd.render_help();
        let help_str = help.to_string();

        assert!(help_str.contains("--container"));
        assert!(help_str.contains("--test-case"));
        assert!(help_str.contains("-o"));
    }

    #[test]
    fn test_parse_with_paths_containing_spaces() {
        let args = Args::try_parse_from([
            "test-plan-doc-gen",
            "--container",
            "my schema.json",
            "my template.tera",
            "my container.json",
            "--test-case",
            "verification methods",
            "test 1.json",
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.container[0], PathBuf::from("my schema.json"));
        assert_eq!(args.test_case[1], PathBuf::from("test 1.json"));
    }

    #[test]
    fn test_parse_with_relative_paths() {
        let args = Args::try_parse_from([
            "test-plan-doc-gen",
            "--container",
            "./schemas/container.json",
            "../templates/container.tera",
            "data/container.json",
            "--test-case",
            "./verification_methods",
            "data/test1.json",
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.container[0], PathBuf::from("./schemas/container.json"));
        assert_eq!(args.test_case[0], PathBuf::from("./verification_methods"));
    }

    #[test]
    fn test_parse_with_absolute_paths() {
        let args = Args::try_parse_from([
            "test-plan-doc-gen",
            "--container",
            "/usr/local/schema.json",
            "/usr/local/template.tera",
            "/usr/local/container.json",
            "--test-case",
            "/usr/local/verification_methods",
            "/usr/local/test1.json",
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.container[0], PathBuf::from("/usr/local/schema.json"));
        assert_eq!(
            args.test_case[0],
            PathBuf::from("/usr/local/verification_methods")
        );
    }

    #[test]
    fn test_requirement_aggregation_logic() {
        let template_str = r#"requirements_with_detail:
{%- set reqs = ["XXX100", "XXX200", "XXX300", "XXX400"] %}
{%- for req in reqs %}
{%- set filtered = test_results | filter(attribute="requirement", value=req) %}
{%- if filtered | length > 0 %}
  - requirement: {{ req }}
    items:
{%- for item in filtered %}
      - item: {% if item.item %}{{ item.item }}{% else %}null{% endif %}
        tc: {% if item.tc %}{{ item.tc }}{% else %}null{% endif %}
        id: {{ item.test_case_id }}
        pass: {% if item.overall_pass %}true{% else %}false{% endif %}
{%- endfor %}
{%- endif %}
{%- endfor %}
status_per_requirement:
{%- for req in reqs %}
{%- set filtered = test_results | filter(attribute="requirement", value=req) %}
{%- if filtered | length > 0 %}
{%- set all_pass = filtered | filter(attribute="overall_pass", value=true) | length == filtered | length %}
  - requirement: {{ req }}
    pass: {% if all_pass %}true{% else %}false{% endif %}
{%- endif %}
{%- endfor %}
requirements_by_status:
  pass:
{%- for req in reqs %}
{%- set filtered = test_results | filter(attribute="requirement", value=req) %}
{%- if filtered | length > 0 %}
{%- set all_pass = filtered | filter(attribute="overall_pass", value=true) | length == filtered | length %}
{%- if all_pass %}
    - {{ req }}
{%- endif %}
{%- endif %}
{%- endfor %}
  fail:
{%- for req in reqs %}
{%- set filtered = test_results | filter(attribute="requirement", value=req) %}
{%- if filtered | length > 0 %}
{%- set all_pass = filtered | filter(attribute="overall_pass", value=true) | length == filtered | length %}
{%- if not all_pass %}
    - {{ req }}
{%- endif %}
{%- endif %}
{%- endfor %}"#;

        let test_data = serde_json::json!([
            {
                "requirement": "XXX100",
                "item": 1,
                "tc": 4,
                "test_case_id": "TC-100-1",
                "overall_pass": true
            },
            {
                "requirement": "XXX100",
                "item": 2,
                "tc": 5,
                "test_case_id": "TC-100-2",
                "overall_pass": false
            },
            {
                "requirement": "XXX200",
                "test_case_id": "TC-200-1",
                "overall_pass": true
            },
            {
                "requirement": "XXX300",
                "item": 1,
                "tc": 1,
                "test_case_id": "TC-300-1",
                "overall_pass": true
            },
            {
                "requirement": "XXX300",
                "item": 2,
                "tc": 2,
                "test_case_id": "TC-300-2",
                "overall_pass": true
            },
            {
                "requirement": "XXX400",
                "test_case_id": "TC-400-1",
                "overall_pass": false
            }
        ]);

        let mut context = Context::new();
        context.insert("test_results", &test_data);

        let rendered = render_template(template_str, &context).expect("Failed to render template");

        let parsed: YamlValue =
            serde_yaml::from_str(&rendered).expect("Failed to parse YAML output");
        let parsed_map = parsed.as_mapping().expect("Expected YAML mapping");

        let requirements_with_detail = parsed_map
            .get(YamlValue::String("requirements_with_detail".to_string()))
            .expect("Missing requirements_with_detail")
            .as_sequence()
            .expect("Expected sequence");

        assert_eq!(requirements_with_detail.len(), 4);

        let req100 = requirements_with_detail[0]
            .as_mapping()
            .expect("Expected mapping");
        assert_eq!(
            req100.get(YamlValue::String("requirement".to_string())),
            Some(&YamlValue::String("XXX100".to_string()))
        );
        let req100_items = req100
            .get(YamlValue::String("items".to_string()))
            .expect("Missing items")
            .as_sequence()
            .expect("Expected sequence");
        assert_eq!(req100_items.len(), 2);
        assert_eq!(
            req100_items[0]
                .as_mapping()
                .unwrap()
                .get(YamlValue::String("item".to_string())),
            Some(&YamlValue::Number(1.into()))
        );
        assert_eq!(
            req100_items[0]
                .as_mapping()
                .unwrap()
                .get(YamlValue::String("tc".to_string())),
            Some(&YamlValue::Number(4.into()))
        );
        assert_eq!(
            req100_items[0]
                .as_mapping()
                .unwrap()
                .get(YamlValue::String("id".to_string())),
            Some(&YamlValue::String("TC-100-1".to_string()))
        );
        assert_eq!(
            req100_items[0]
                .as_mapping()
                .unwrap()
                .get(YamlValue::String("pass".to_string())),
            Some(&YamlValue::Bool(true))
        );
        assert_eq!(
            req100_items[1]
                .as_mapping()
                .unwrap()
                .get(YamlValue::String("pass".to_string())),
            Some(&YamlValue::Bool(false))
        );

        let req200 = requirements_with_detail[1]
            .as_mapping()
            .expect("Expected mapping");
        assert_eq!(
            req200.get(YamlValue::String("requirement".to_string())),
            Some(&YamlValue::String("XXX200".to_string()))
        );
        let req200_items = req200
            .get(YamlValue::String("items".to_string()))
            .expect("Missing items")
            .as_sequence()
            .expect("Expected sequence");
        assert_eq!(req200_items.len(), 1);
        assert_eq!(
            req200_items[0]
                .as_mapping()
                .unwrap()
                .get(YamlValue::String("item".to_string())),
            Some(&YamlValue::Null)
        );
        assert_eq!(
            req200_items[0]
                .as_mapping()
                .unwrap()
                .get(YamlValue::String("pass".to_string())),
            Some(&YamlValue::Bool(true))
        );

        let req300 = requirements_with_detail[2]
            .as_mapping()
            .expect("Expected mapping");
        assert_eq!(
            req300.get(YamlValue::String("requirement".to_string())),
            Some(&YamlValue::String("XXX300".to_string()))
        );
        let req300_items = req300
            .get(YamlValue::String("items".to_string()))
            .expect("Missing items")
            .as_sequence()
            .expect("Expected sequence");
        assert_eq!(req300_items.len(), 2);
        assert_eq!(
            req300_items[0]
                .as_mapping()
                .unwrap()
                .get(YamlValue::String("pass".to_string())),
            Some(&YamlValue::Bool(true))
        );
        assert_eq!(
            req300_items[1]
                .as_mapping()
                .unwrap()
                .get(YamlValue::String("pass".to_string())),
            Some(&YamlValue::Bool(true))
        );

        let status_per_requirement = parsed_map
            .get(YamlValue::String("status_per_requirement".to_string()))
            .expect("Missing status_per_requirement")
            .as_sequence()
            .expect("Expected sequence");

        assert_eq!(status_per_requirement.len(), 4);

        let status100 = status_per_requirement[0]
            .as_mapping()
            .expect("Expected mapping");
        assert_eq!(
            status100.get(YamlValue::String("requirement".to_string())),
            Some(&YamlValue::String("XXX100".to_string()))
        );
        assert_eq!(
            status100.get(YamlValue::String("pass".to_string())),
            Some(&YamlValue::Bool(false))
        );

        let status200 = status_per_requirement[1]
            .as_mapping()
            .expect("Expected mapping");
        assert_eq!(
            status200.get(YamlValue::String("requirement".to_string())),
            Some(&YamlValue::String("XXX200".to_string()))
        );
        assert_eq!(
            status200.get(YamlValue::String("pass".to_string())),
            Some(&YamlValue::Bool(true))
        );

        let status300 = status_per_requirement[2]
            .as_mapping()
            .expect("Expected mapping");
        assert_eq!(
            status300.get(YamlValue::String("requirement".to_string())),
            Some(&YamlValue::String("XXX300".to_string()))
        );
        assert_eq!(
            status300.get(YamlValue::String("pass".to_string())),
            Some(&YamlValue::Bool(true))
        );

        let status400 = status_per_requirement[3]
            .as_mapping()
            .expect("Expected mapping");
        assert_eq!(
            status400.get(YamlValue::String("requirement".to_string())),
            Some(&YamlValue::String("XXX400".to_string()))
        );
        assert_eq!(
            status400.get(YamlValue::String("pass".to_string())),
            Some(&YamlValue::Bool(false))
        );

        let requirements_by_status = parsed_map
            .get(YamlValue::String("requirements_by_status".to_string()))
            .expect("Missing requirements_by_status")
            .as_mapping()
            .expect("Expected mapping");

        let pass_reqs = requirements_by_status
            .get(YamlValue::String("pass".to_string()))
            .expect("Missing pass")
            .as_sequence()
            .expect("Expected sequence");
        assert_eq!(pass_reqs.len(), 2);
        assert_eq!(pass_reqs[0], YamlValue::String("XXX200".to_string()));
        assert_eq!(pass_reqs[1], YamlValue::String("XXX300".to_string()));

        let fail_reqs = requirements_by_status
            .get(YamlValue::String("fail".to_string()))
            .expect("Missing fail")
            .as_sequence()
            .expect("Expected sequence");
        assert_eq!(fail_reqs.len(), 2);
        assert_eq!(fail_reqs[0], YamlValue::String("XXX100".to_string()));
        assert_eq!(fail_reqs[1], YamlValue::String("XXX400".to_string()));
    }
}
