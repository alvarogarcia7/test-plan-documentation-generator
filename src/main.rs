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
    let schema_str = fs::read_to_string(container_schema)?;
    let schema_json: JsonValue = serde_json::from_str(&schema_str)?;
    // JSONSchema::compile requires the schema to live for 'static because the
    // compiled schema holds references into it. Allocate the schema on the heap
    // and leak it to get a &'static reference. This is acceptable for a short-
    // lived CLI process.
    let schema_box = Box::new(schema_json);
    let schema_static: &'static JsonValue = Box::leak(schema_box);
    let compiled = JSONSchema::compile(schema_static).map_err(|e| anyhow::anyhow!(e))?;
    log_fd3!("Validating container file against schema...");
    if let Err(errors) = compiled.validate(&json_value) {
        log_fd3!("Schema validation: INVALID");
        eprintln!("Error: JSON Schema validation failed for container file:");
        for err in errors {
            eprintln!(" - {}", err);
        }
        exit(3);
    } else {
        log_fd3!("Schema validation: VALID");
    }
    log_fd3!("Validation successful.");

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
        println!(
            "Template rendered successfully to {}",
            output_path.display()
        );
        log_fd3!("Rendered output, writing to {:?}", output_path);
    } else {
        log_fd3!("Rendering container to console. You can .... 3>log_fd3.txt to ignore the file descriptor 3");
        println!("{}", rendered);
    }

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
    use insta::assert_snapshot;
    use std::fs::File;
    use std::io::Write;
    use std::process::Command;
    use tempfile::tempdir;

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

    // End-to-End Tests

    fn get_binary_path() -> PathBuf {
        // Get the path to the compiled binary
        let mut path = std::env::current_exe().unwrap();
        path.pop(); // remove test binary name
        if path.ends_with("deps") {
            path.pop(); // remove deps directory
        }
        path.push("test-plan-doc-gen");
        path
    }

    #[test]
    fn test_e2e_basic_yaml_rendering() {
        let dir = tempdir().unwrap();
        let yaml_path = dir.path().join("data.yaml");
        let template_path = dir.path().join("template.tera");
        let output_path = dir.path().join("output.txt");

        // Write a simple YAML file
        let mut yaml_file = File::create(&yaml_path).unwrap();
        writeln!(yaml_file, "name: InstaTest\nage: 42").unwrap();

        // Write a simple Tera template
        let mut template_file = File::create(&template_path).unwrap();
        writeln!(template_file, "Name: {{{{ name }}}}\nAge: {{{{ age }}}}").unwrap();

        // Create dummy schema files (not used in logic)
        let schema_path = dir.path().join("schema.json");
        std::fs::write(&schema_path, "{}").unwrap();
        let tc_schema_path = dir.path().join("tc_schema.json");
        std::fs::write(&tc_schema_path, "{}").unwrap();
        let tc_template_path = dir.path().join("tc_template.tera");
        File::create(&tc_template_path).unwrap();
        let tc_file_path = dir.path().join("tc_file.json");
        File::create(&tc_file_path).unwrap();

        // Run the binary as a subprocess
        let status = Command::new(get_binary_path())
            .arg("--container")
            .arg(schema_path.to_str().unwrap())
            .arg(template_path.to_str().unwrap())
            .arg(yaml_path.to_str().unwrap())
            .arg("--test-case")
            .arg(tc_schema_path.to_str().unwrap())
            .arg(tc_template_path.to_str().unwrap())
            .arg(tc_file_path.to_str().unwrap())
            .arg("-o")
            .arg(output_path.to_str().unwrap())
            .status()
            .expect("failed to run binary");
        assert!(status.success());

        assert!(output_path.exists(), "Output file was not created");
        // Read the output and snapshot it
        let output = fs::read_to_string(&output_path).unwrap();
        assert_snapshot!("e2e_basic_yaml_rendering", output);
    }

    #[test]
    fn test_e2e_stdout_output() {
        let dir = tempdir().unwrap();
        let yaml_path = dir.path().join("data.yaml");
        let template_path = dir.path().join("template.tera");

        // Write YAML file
        let mut yaml_file = File::create(&yaml_path).unwrap();
        writeln!(yaml_file, "title: Test Report\nversion: 1.0").unwrap();

        // Write template
        let mut template_file = File::create(&template_path).unwrap();
        writeln!(
            template_file,
            "# {{{{ title }}}}\nVersion: {{{{ version }}}}"
        )
        .unwrap();

        // Create dummy files
        let schema_path = dir.path().join("schema.json");
        std::fs::write(&schema_path, "{}").unwrap();
        let tc_schema_path = dir.path().join("tc_schema.json");
        std::fs::write(&tc_schema_path, "{}").unwrap();
        let tc_template_path = dir.path().join("tc_template.tera");
        File::create(&tc_template_path).unwrap();
        let tc_file_path = dir.path().join("tc_file.json");
        File::create(&tc_file_path).unwrap();

        // Run without -o flag to test stdout output
        let output = Command::new(get_binary_path())
            .arg("--container")
            .arg(schema_path.to_str().unwrap())
            .arg(template_path.to_str().unwrap())
            .arg(yaml_path.to_str().unwrap())
            .arg("--test-case")
            .arg(tc_schema_path.to_str().unwrap())
            .arg(tc_template_path.to_str().unwrap())
            .arg(tc_file_path.to_str().unwrap())
            .output()
            .expect("failed to run binary");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_snapshot!("e2e_stdout_output", stdout);
    }

    #[test]
    fn test_e2e_complex_yaml_structure() {
        let dir = tempdir().unwrap();
        let yaml_path = dir.path().join("data.yaml");
        let template_path = dir.path().join("template.tera");
        let output_path = dir.path().join("output.txt");

        // Write complex YAML with nested structures
        let mut yaml_file = File::create(&yaml_path).unwrap();
        writeln!(yaml_file, "project: TestSuite").unwrap();
        writeln!(yaml_file, "version: 2.0").unwrap();
        writeln!(yaml_file, "features:").unwrap();
        writeln!(yaml_file, "  - login").unwrap();
        writeln!(yaml_file, "  - logout").unwrap();
        writeln!(yaml_file, "  - registration").unwrap();
        writeln!(yaml_file, "metadata:").unwrap();
        writeln!(yaml_file, "  author: TestTeam").unwrap();
        writeln!(yaml_file, "  date: 2025-12-12").unwrap();

        // Write template using complex structures
        let mut template_file = File::create(&template_path).unwrap();
        writeln!(template_file, "# {{{{ project }}}} v{{{{ version }}}}\n").unwrap();
        writeln!(template_file, "## Features").unwrap();
        writeln!(template_file, "{{% for feature in features %}}").unwrap();
        writeln!(template_file, "- {{{{ feature }}}}").unwrap();
        writeln!(template_file, "{{% endfor %}}\n").unwrap();
        writeln!(template_file, "Author: {{{{ metadata.author }}}}").unwrap();
        writeln!(template_file, "Date: {{{{ metadata.date }}}}").unwrap();

        // Create dummy files
        let schema_path = dir.path().join("schema.json");
        std::fs::write(&schema_path, "{}").unwrap();
        let tc_schema_path = dir.path().join("tc_schema.json");
        std::fs::write(&tc_schema_path, "{}").unwrap();
        let tc_template_path = dir.path().join("tc_template.tera");
        File::create(&tc_template_path).unwrap();
        let tc_file_path = dir.path().join("tc_file.json");
        File::create(&tc_file_path).unwrap();

        let status = Command::new(get_binary_path())
            .arg("--container")
            .arg(schema_path.to_str().unwrap())
            .arg(template_path.to_str().unwrap())
            .arg(yaml_path.to_str().unwrap())
            .arg("--test-case")
            .arg(tc_schema_path.to_str().unwrap())
            .arg(tc_template_path.to_str().unwrap())
            .arg(tc_file_path.to_str().unwrap())
            .arg("-o")
            .arg(output_path.to_str().unwrap())
            .status()
            .expect("failed to run binary");

        assert!(status.success());
        assert!(output_path.exists(), "Output file was not created");
        let output = std::fs::read_to_string(&output_path).unwrap();
        assert_snapshot!("e2e_complex_yaml_structure", output);
    }

    #[test]
    fn test_e2e_multiple_test_case_files() {
        let dir = tempdir().unwrap();
        let yaml_path = dir.path().join("data.yaml");
        let template_path = dir.path().join("template.tera");
        let output_path = dir.path().join("output.txt");

        // Write YAML
        let mut yaml_file = File::create(&yaml_path).unwrap();
        writeln!(yaml_file, "status: complete").unwrap();

        // Write template
        let mut template_file = File::create(&template_path).unwrap();
        writeln!(template_file, "Status: {{{{ status }}}}").unwrap();

        // Create dummy files including multiple test case files
        let schema_path = dir.path().join("schema.json");
        std::fs::write(&schema_path, "{}").unwrap();
        let tc_schema_path = dir.path().join("tc_schema.json");
        std::fs::write(&tc_schema_path, "{}").unwrap();
        let tc_template_path = dir.path().join("tc_template.tera");
        File::create(&tc_template_path).unwrap();
        let tc_file1 = dir.path().join("tc_file1.json");
        File::create(&tc_file1).unwrap();
        let tc_file2 = dir.path().join("tc_file2.json");
        File::create(&tc_file2).unwrap();
        let tc_file3 = dir.path().join("tc_file3.json");
        File::create(&tc_file3).unwrap();

        let status = Command::new(get_binary_path())
            .arg("--container")
            .arg(schema_path.to_str().unwrap())
            .arg(template_path.to_str().unwrap())
            .arg(yaml_path.to_str().unwrap())
            .arg("--test-case")
            .arg(tc_schema_path.to_str().unwrap())
            .arg(tc_template_path.to_str().unwrap())
            .arg(tc_file1.to_str().unwrap())
            .arg(tc_file2.to_str().unwrap())
            .arg(tc_file3.to_str().unwrap())
            .arg("-o")
            .arg(output_path.to_str().unwrap())
            .status()
            .expect("failed to run binary");

        assert!(status.success());
        assert!(output_path.exists(), "Output file was not created");
        let output = std::fs::read_to_string(&output_path).unwrap();
        assert_snapshot!("e2e_multiple_test_case_files", output);
    }

    #[test]
    fn test_e2e_missing_container_file() {
        let dir = tempdir().unwrap();
        let yaml_path = dir.path().join("nonexistent.yaml");
        let template_path = dir.path().join("template.tera");

        // Create template but not YAML
        let mut template_file = File::create(&template_path).unwrap();
        writeln!(template_file, "Test").unwrap();

        // Create dummy files
        let schema_path = dir.path().join("schema.json");
        std::fs::write(&schema_path, "{}").unwrap();
        let tc_schema_path = dir.path().join("tc_schema.json");
        std::fs::write(&tc_schema_path, "{}").unwrap();
        let tc_template_path = dir.path().join("tc_template.tera");
        File::create(&tc_template_path).unwrap();
        let tc_file_path = dir.path().join("tc_file.json");
        File::create(&tc_file_path).unwrap();

        let output = Command::new(get_binary_path())
            .arg("--container")
            .arg(schema_path.to_str().unwrap())
            .arg(template_path.to_str().unwrap())
            .arg(yaml_path.to_str().unwrap())
            .arg("--test-case")
            .arg(tc_schema_path.to_str().unwrap())
            .arg(tc_template_path.to_str().unwrap())
            .arg(tc_file_path.to_str().unwrap())
            .output()
            .expect("failed to run binary");

        assert!(!output.status.success());
        assert_eq!(output.status.code(), Some(2));
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("do not exist"));
    }

    #[test]
    fn test_e2e_missing_template_file() {
        let dir = tempdir().unwrap();
        let yaml_path = dir.path().join("data.yaml");
        let template_path = dir.path().join("nonexistent.tera");

        // Create YAML but not template
        let mut yaml_file = File::create(&yaml_path).unwrap();
        writeln!(yaml_file, "test: value").unwrap();

        // Create dummy files
        let schema_path = dir.path().join("schema.json");
        std::fs::write(&schema_path, "{}").unwrap();
        let tc_schema_path = dir.path().join("tc_schema.json");
        std::fs::write(&tc_schema_path, "{}").unwrap();
        let tc_template_path = dir.path().join("tc_template.tera");
        File::create(&tc_template_path).unwrap();
        let tc_file_path = dir.path().join("tc_file.json");
        File::create(&tc_file_path).unwrap();

        let output = Command::new(get_binary_path())
            .arg("--container")
            .arg(schema_path.to_str().unwrap())
            .arg(template_path.to_str().unwrap())
            .arg(yaml_path.to_str().unwrap())
            .arg("--test-case")
            .arg(tc_schema_path.to_str().unwrap())
            .arg(tc_template_path.to_str().unwrap())
            .arg(tc_file_path.to_str().unwrap())
            .output()
            .expect("failed to run binary");

        assert!(!output.status.success());
        assert_eq!(output.status.code(), Some(2));
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("do not exist"));
    }

    #[test]
    fn test_e2e_invalid_yaml() {
        let dir = tempdir().unwrap();
        let yaml_path = dir.path().join("data.yaml");
        let template_path = dir.path().join("template.tera");
        let output_path = dir.path().join("output.txt");

        // Write invalid YAML
        let mut yaml_file = File::create(&yaml_path).unwrap();
        writeln!(yaml_file, "invalid: yaml: content: [").unwrap();

        // Write template
        let mut template_file = File::create(&template_path).unwrap();
        writeln!(template_file, "Test").unwrap();

        // Create dummy files
        let schema_path = dir.path().join("schema.json");
        std::fs::write(&schema_path, "{}").unwrap();
        let tc_schema_path = dir.path().join("tc_schema.json");
        std::fs::write(&tc_schema_path, "{}").unwrap();
        let tc_template_path = dir.path().join("tc_template.tera");
        File::create(&tc_template_path).unwrap();
        let tc_file_path = dir.path().join("tc_file.json");
        File::create(&tc_file_path).unwrap();

        let output = Command::new(get_binary_path())
            .arg("--container")
            .arg(schema_path.to_str().unwrap())
            .arg(template_path.to_str().unwrap())
            .arg(yaml_path.to_str().unwrap())
            .arg("--test-case")
            .arg(tc_schema_path.to_str().unwrap())
            .arg(tc_template_path.to_str().unwrap())
            .arg(tc_file_path.to_str().unwrap())
            .arg("-o")
            .arg(output_path.to_str().unwrap())
            .output()
            .expect("failed to run binary");

        assert!(!output.status.success());
    }

    #[test]
    fn test_e2e_invalid_template_syntax() {
        let dir = tempdir().unwrap();
        let yaml_path = dir.path().join("data.yaml");
        let template_path = dir.path().join("template.tera");
        let output_path = dir.path().join("output.txt");

        // Write valid YAML
        let mut yaml_file = File::create(&yaml_path).unwrap();
        writeln!(yaml_file, "name: Test").unwrap();

        // Write invalid template - unclosed variable bracket
        let mut template_file = File::create(&template_path).unwrap();
        writeln!(template_file, "{{{{ unclosed").unwrap();

        // Create dummy files
        let schema_path = dir.path().join("schema.json");
        std::fs::write(&schema_path, "{}").unwrap();
        let tc_schema_path = dir.path().join("tc_schema.json");
        std::fs::write(&tc_schema_path, "{}").unwrap();
        let tc_template_path = dir.path().join("tc_template.tera");
        File::create(&tc_template_path).unwrap();
        let tc_file_path = dir.path().join("tc_file.json");
        File::create(&tc_file_path).unwrap();

        let output = Command::new(get_binary_path())
            .arg("--container")
            .arg(schema_path.to_str().unwrap())
            .arg(template_path.to_str().unwrap())
            .arg(yaml_path.to_str().unwrap())
            .arg("--test-case")
            .arg(tc_schema_path.to_str().unwrap())
            .arg(tc_template_path.to_str().unwrap())
            .arg(tc_file_path.to_str().unwrap())
            .arg("-o")
            .arg(output_path.to_str().unwrap())
            .output()
            .expect("failed to run binary");

        assert!(!output.status.success());
    }

    #[test]
    fn test_e2e_empty_yaml() {
        let dir = tempdir().unwrap();
        let yaml_path = dir.path().join("data.yaml");
        let template_path = dir.path().join("template.tera");
        let output_path = dir.path().join("output.txt");

        // Write empty YAML
        File::create(&yaml_path).unwrap();

        // Write template without variables
        let mut template_file = File::create(&template_path).unwrap();
        writeln!(template_file, "Static Content Only").unwrap();

        // Create dummy files
        let schema_path = dir.path().join("schema.json");
        std::fs::write(&schema_path, "{}").unwrap();
        let tc_schema_path = dir.path().join("tc_schema.json");
        std::fs::write(&tc_schema_path, "{}").unwrap();
        let tc_template_path = dir.path().join("tc_template.tera");
        File::create(&tc_template_path).unwrap();
        let tc_file_path = dir.path().join("tc_file.json");
        File::create(&tc_file_path).unwrap();

        let status = Command::new(get_binary_path())
            .arg("--container")
            .arg(schema_path.to_str().unwrap())
            .arg(template_path.to_str().unwrap())
            .arg(yaml_path.to_str().unwrap())
            .arg("--test-case")
            .arg(tc_schema_path.to_str().unwrap())
            .arg(tc_template_path.to_str().unwrap())
            .arg(tc_file_path.to_str().unwrap())
            .arg("-o")
            .arg(output_path.to_str().unwrap())
            .status()
            .expect("failed to run binary");

        assert!(status.success());
        assert!(output_path.exists(), "Output file was not created");
        let output = std::fs::read_to_string(&output_path).unwrap();
        assert_snapshot!("e2e_empty_yaml", output);
    }

    #[test]
    fn test_e2e_template_with_filters() {
        let dir = tempdir().unwrap();
        let yaml_path = dir.path().join("data.yaml");
        let template_path = dir.path().join("template.tera");
        let output_path = dir.path().join("output.txt");

        // Write YAML
        let mut yaml_file = File::create(&yaml_path).unwrap();
        writeln!(yaml_file, "name: test user").unwrap();
        writeln!(yaml_file, "count: 42").unwrap();

        // Write template with Tera filters
        let mut template_file = File::create(&template_path).unwrap();
        writeln!(template_file, "Name: {{{{ name | upper }}}}").unwrap();
        writeln!(template_file, "Count: {{{{ count }}}}").unwrap();

        // Create dummy files
        let schema_path = dir.path().join("schema.json");
        std::fs::write(&schema_path, "{}").unwrap();
        let tc_schema_path = dir.path().join("tc_schema.json");
        std::fs::write(&tc_schema_path, "{}").unwrap();
        let tc_template_path = dir.path().join("tc_template.tera");
        File::create(&tc_template_path).unwrap();
        let tc_file_path = dir.path().join("tc_file.json");
        File::create(&tc_file_path).unwrap();

        let status = Command::new(get_binary_path())
            .arg("--container")
            .arg(schema_path.to_str().unwrap())
            .arg(template_path.to_str().unwrap())
            .arg(yaml_path.to_str().unwrap())
            .arg("--test-case")
            .arg(tc_schema_path.to_str().unwrap())
            .arg(tc_template_path.to_str().unwrap())
            .arg(tc_file_path.to_str().unwrap())
            .arg("-o")
            .arg(output_path.to_str().unwrap())
            .status()
            .expect("failed to run binary");

        assert!(status.success());
        assert!(output_path.exists(), "Output file was not created");
        let output = std::fs::read_to_string(&output_path).unwrap();
        assert_snapshot!("e2e_template_with_filters", output);
    }

    #[test]
    fn test_e2e_dataset_4_gsma() {
        // Run the binary with the real dataset paths (expand test-case files from dir)
        let bin = get_binary_path();

        // Compose the fixed container args
        let container_schema = "./data/dataset_4_GSMA/container/schema.json";
        let container_template = "./data/dataset_4_GSMA/container/template.j2";
        let container_file = "./data/dataset_4_GSMA/container/data.yml";

        // Test-case schema and template
        let tc_schema = "./data/dataset_4_GSMA/test_case/schema.json";
        let tc_template = "./data/dataset_4_GSMA/test_case/template.j2";

        // Expand all .yml files under the test_case directory
        let mut tc_files: Vec<String> = Vec::new();
        let tc_dir = std::path::Path::new("./data/dataset_4_GSMA/test_case");
        for entry in std::fs::read_dir(tc_dir).expect("failed to read test_case directory") {
            let entry = entry.expect("failed to read dir entry");
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "yml" || ext == "yaml" {
                        tc_files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }

        assert!(
            !tc_files.is_empty(),
            "no test-case yml files found in dataset_4_GSMA/test_case"
        );

        // Build args vector
        let mut cmd = Command::new(bin);
        cmd.arg("--container")
            .arg(container_schema)
            .arg(container_template)
            .arg(container_file);
        cmd.arg("--test-case").arg(tc_schema).arg(tc_template);
        for f in &tc_files {
            cmd.arg(f);
        }
        // Use a tempdir to avoid interfering with parallel tests
        let td = tempdir().unwrap();
        let report_path = td.path().join("report.md");
        cmd.arg("-o").arg(report_path.as_os_str());

        // Execute
        let status = cmd.status().expect("failed to execute test-plan-doc-gen");
        assert!(status.success(), "binary exited with non-zero status");

        // Verify report.md was created and is not empty
        assert!(report_path.exists(), "report.md was not created");
        let metadata = std::fs::metadata(&report_path).expect("failed to stat report.md");
        assert!(metadata.len() > 0, "report.md is empty");

        // Read content, sanitize dynamic paths, and snapshot
        let mut output =
            std::fs::read_to_string(&report_path).expect("failed to read generated report.md");
        let tmp_prefix = std::env::temp_dir().to_string_lossy().to_string();
        if !tmp_prefix.is_empty() {
            output = output.replace(&tmp_prefix, "<TMPDIR>");
        }
        let cwd = std::env::current_dir()
            .expect("cwd")
            .to_string_lossy()
            .to_string();
        if !cwd.is_empty() {
            output = output.replace(&cwd, "<CWD>");
        }
        assert_snapshot!("e2e_dataset_4_gsma", output);

        // tempdir will be cleaned up when `td` goes out of scope
    }

    #[test]
    fn test_e2e_dataset_4_gsma_target_debug() {
        // Run the same scenario but invoking the explicit binary path under target/debug
        let bin_path = std::path::Path::new("target/debug/test-plan-doc-gen");
        if !bin_path.exists() {
            // Skip the test if that binary isn't built in this environment.
            eprintln!(
                "Skipping test_e2e_dataset_4_gsma_target_debug: {} not found",
                bin_path.display()
            );
            return;
        }

        // Compose the fixed container args
        let container_schema = "./data/dataset_4_GSMA/container/schema.json";
        let container_template = "./data/dataset_4_GSMA/container/template.j2";
        let container_file = "./data/dataset_4_GSMA/container/data.yml";

        // Test-case schema and template
        let tc_schema = "./data/dataset_4_GSMA/test_case/schema.json";
        let tc_template = "./data/dataset_4_GSMA/test_case/template.j2";

        // Expand all .yml files under the test_case directory
        let mut tc_files: Vec<String> = Vec::new();
        let tc_dir = std::path::Path::new("./data/dataset_4_GSMA/test_case");
        for entry in std::fs::read_dir(tc_dir).expect("failed to read test_case directory") {
            let entry = entry.expect("failed to read dir entry");
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "yml" || ext == "yaml" {
                        tc_files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }

        assert!(
            !tc_files.is_empty(),
            "no test-case yml files found in dataset_4_GSMA/test_case"
        );

        // Build args vector and run
        let mut cmd = std::process::Command::new(bin_path);
        cmd.arg("--container")
            .arg(container_schema)
            .arg(container_template)
            .arg(container_file);
        cmd.arg("--test-case").arg(tc_schema).arg(tc_template);
        for f in &tc_files {
            cmd.arg(f);
        }
        // Use a tempdir to avoid interfering with parallel tests
        let td = tempdir().unwrap();
        let report_path = td.path().join("report.md");
        cmd.arg("-o").arg(report_path.as_os_str());
        // Ensure the child runs from the project root (where tests run) so
        // relative output paths are predictable.
        let cwd = std::env::current_dir().expect("can't get current dir");
        cmd.current_dir(&cwd);

        let status = cmd
            .status()
            .expect("failed to execute target/debug/test-plan-doc-gen");
        assert!(status.success(), "binary exited with non-zero status");

        // Some binaries write in their working dir; check both locations.
        if !report_path.exists() {
            let alt = bin_path
                .parent()
                .unwrap_or(std::path::Path::new("."))
                .join("report.md");
            if alt.exists() {
                std::fs::rename(&alt, &report_path)
                    .expect("failed to move report.md from binary dir");
            }
        }

        assert!(report_path.exists(), "report.md was not created");
        let metadata = std::fs::metadata(&report_path).expect("failed to stat report.md");
        assert!(metadata.len() > 0, "report.md is empty");

        // Read content, sanitize dynamic paths, and snapshot
        let mut output =
            std::fs::read_to_string(&report_path).expect("failed to read generated report.md");
        let tmp_prefix = std::env::temp_dir().to_string_lossy().to_string();
        if !tmp_prefix.is_empty() {
            output = output.replace(&tmp_prefix, "<TMPDIR>");
        }
        let cwd = std::env::current_dir()
            .expect("cwd")
            .to_string_lossy()
            .to_string();
        if !cwd.is_empty() {
            output = output.replace(&cwd, "<CWD>");
        }
        assert_snapshot!("e2e_dataset_4_gsma_target_debug", output);

        // tempdir will be removed automatically
    }
}
