use anyhow::Result;
use clap::Parser;
use jsonschema::JSONSchema;
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
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

    /// Test case schema and files
    #[arg(long = "test-case", value_names = ["TEST_CASE_SCHEMA", "TEST_CASE_TEMPLATE", "TEST_CASE_FILE", "REST_TEST_CASE_FILES"], num_args = 3.., required = true)]
    test_case: Vec<PathBuf>,
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

    // Parse test case arguments
    let test_case_schema = &args.test_case[0];
    let test_case_template = if args.test_case.len() > 1 {
        Some(&args.test_case[1])
    } else {
        usage(1)
    };
    let test_case_file = if args.test_case.len() > 2 {
        Some(&args.test_case[2])
    } else {
        usage(1)
    };
    let rest_test_case_files = if args.test_case.len() > 3 {
        &args.test_case[3..]
    } else {
        &[]
    };

    // Verify that all received files exist
    let mut missing_files = Vec::new();
    let all_files = [
        vec![container_schema, container_template, container_file],
        vec![test_case_schema],
        test_case_template.iter().copied().collect::<Vec<_>>(),
        test_case_file.iter().copied().collect::<Vec<_>>(),
        rest_test_case_files.iter().collect::<Vec<_>>(),
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

    // --- Render all test-case files into a temporary `output.md` ---
    // Read and compile the test-case template
    let test_case_template_path = test_case_template.expect("test_case_template must exist");
    log_fd3!(
        "Loading test-case template from: {}",
        test_case_template_path.display()
    );
    let tc_template_str = fs::read_to_string(test_case_template_path)?;
    let mut tc_tera = Tera::default();
    tc_tera.add_raw_template("tc_template", &tc_template_str)?;

    // Collect all test case files (the first is test_case_file, the rest follow)
    let mut tc_files: Vec<&PathBuf> = Vec::new();
    if let Some(p) = test_case_file {
        tc_files.push(p);
    }
    for p in rest_test_case_files {
        tc_files.push(p);
    }

    for file in &tc_files {
        log_fd3!("Validating test-case file: {}", file.display());

        log_fd3!(
            "\tLoading test-case data for validation from: {}",
            file.display()
        );
        let content = fs::read_to_string(file)?;
        let yaml_val: YamlValue =
            serde_yaml::from_str(&content).unwrap_or_else(|_| YamlValue::Null);
        let json_value: JsonValue = serde_json::from_str(&serde_json::to_string(&yaml_val)?)?;

        let validation_result: Result<(), Vec<String>> =
            validate_json_schema(test_case_schema, &json_value);
        match validation_result {
            Ok(_) => {
                log_fd3!("\tValidation successful.");
            }
            Err(errors) => {
                log_fd3!("\tValidation failed.");
                for error in errors {
                    log_fd3!("\tValidation error: {}", error);
                }
                usage(3);
            }
        }
    }
    log_fd3!("All test-case files validated successfully");

    let mut concatenated = String::new();
    let mut first = true;
    for file in &tc_files {
        log_fd3!("Loading test-case data from: {}", file.display());
        let content = fs::read_to_string(file)?;
        // Try to parse as YAML; if parsing fails, treat it as empty context
        let yaml_val: YamlValue =
            serde_yaml::from_str(&content).unwrap_or_else(|_| YamlValue::Null);

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
        let json_value_full: JsonValue = serde_json::from_str(&serde_json::to_string(&yaml_val)?)?;
        tc_context.insert("data", &json_value_full);

        // Render
        log_fd3!("Rendering test-case template for: {}", file.display());
        let rendered = tc_tera.render("tc_template", &tc_context)?;
        if !first {
            concatenated.push_str("\n\n");
        }
        first = false;
        concatenated.push_str(&rendered);
    }
    log_fd3!("Rendering test-case files: {:?}", tc_files);

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
    let validation_result: Result<(), Vec<String>> =
        validate_json_schema(container_schema, &json_value);
    match validation_result {
        Ok(_) => {
            log_fd3!("Validation successful.");
        }
        Err(errors) => {
            log_fd3!("Validation failed.");
            for error in errors {
                log_fd3!("Validation error: {}", error);
            }
            usage(3);
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
            log_fd3!(
                "\tFailed to read schema file {}: {}",
                schema_path.display(),
                e
            );
            return Err(vec![]); // Return empty vector on read error
        }
    };
    let schema_str_2 = schema_str;
    let schema_json = serde_json::from_str(&schema_str_2);
    let schema_json = match schema_json {
        Ok(s) => s,
        Err(e) => {
            log_fd3!(
                "Failed to parse schema file {}: {}",
                schema_path.display(),
                e
            );
            return Err(vec![]); // Return empty vector on parse error
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
            log_fd3!(
                "\tFailed to compile schema from file {}: {}",
                schema_path.display(),
                e
            );
            return Err(vec![]); // Return empty vector on compile error
        }
    };
    log_fd3!("\tValidating payload file against schema...");
    match compiled2.validate(payload) {
        Ok(_) => log_fd3!("\tSchema validation: VALID"),
        Err(errors) => {
            log_fd3!("\tSchema validation: INVALID");
            return Err(errors.into_iter().map(|e| e.to_string()).collect());
        }
    }
    log_fd3!("\tValidation successful.");
    Ok(())
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
            "tc_schema.json",
            "tc_template.tera",
            "test1.json",
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.container.len(), 3);
        assert_eq!(args.test_case.len(), 3);
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
            "tc_schema.json",
            "tc_template.tera",
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
            "tc_schema.json",
            "tc_template.tera",
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
            "tc_schema.json",
            "tc_template.tera",
            "test1.json",
            "test2.json",
            "test3.json",
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.test_case.len(), 5); // schema + template + 3 files
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
            "tc_schema.json",
            "tc_template.tera",
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
            "tc_schema.json",
            "tc_template.tera",
            "test1.json",
            "test2.json",
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.test_case[0], PathBuf::from("tc_schema.json"));
        assert_eq!(args.test_case[1], PathBuf::from("tc_template.tera"));
        assert_eq!(args.test_case[2], PathBuf::from("test1.json"));
        assert_eq!(args.test_case[3], PathBuf::from("test2.json"));
    }

    #[test]
    fn test_parse_missing_container_arg() {
        let args = Args::try_parse_from([
            "test-plan-doc-gen",
            "--test-case",
            "tc_schema.json",
            "tc_template.tera",
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
            "tc_schema.json",
            "tc_template.tera",
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
            "tc_schema.json",
            "tc_template.tera",
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
            "tc_schema.json",
            "tc_template.tera",
            "test1.json",
        ]);

        assert!(args.is_err());
    }

    #[test]
    fn test_parse_args_order_independence() {
        let args = Args::try_parse_from([
            "test-plan-doc-gen",
            "--test-case",
            "tc_schema.json",
            "tc_template.tera",
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
        assert_eq!(args.test_case.len(), 3);
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
            "tc schema.json",
            "tc template.tera",
            "test 1.json",
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.container[0], PathBuf::from("my schema.json"));
        assert_eq!(args.test_case[2], PathBuf::from("test 1.json"));
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
            "./schemas/tc.json",
            "../templates/tc.tera",
            "data/test1.json",
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.container[0], PathBuf::from("./schemas/container.json"));
        assert_eq!(args.test_case[1], PathBuf::from("../templates/tc.tera"));
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
            "/usr/local/tc_schema.json",
            "/usr/local/tc_template.tera",
            "/usr/local/test1.json",
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.container[0], PathBuf::from("/usr/local/schema.json"));
    }
}
