use anyhow::Result;
use clap::Parser;
use clap::ValueEnum;
use jsonschema::JSONSchema;
use log::{debug, error, info};
use regex::Regex;
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;
use tera::Context;
use tera::Tera;
use tera::{Filter, Function, Value};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    #[value(name = "md")]
    Markdown,
    #[value(name = "adoc")]
    Asciidoc,
}

impl OutputFormat {
    fn template_suffix(&self) -> &'static str {
        match self {
            OutputFormat::Markdown => ".j2",
            OutputFormat::Asciidoc => "_asciidoc.adoc",
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "tpdg")]
#[command(about = "A CLI tool to render templates for test plan documentation", long_about = None)]
struct Args {
    /// Output file path
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Output format (md or adoc)
    #[arg(long = "format", default_value = "adoc")]
    format: OutputFormat,

    /// Single mode: validate and render a single template
    #[arg(long = "single", value_names = ["SCHEMA", "TEMPLATE", "INPUT"], num_args = 3, required_unless_present_any = ["multiple", "multiple_by_type"])]
    single: Vec<PathBuf>,

    /// Multiple mode: validate and render multiple input files with a single schema and template
    #[arg(long = "multiple", value_names = ["SCHEMA", "TEMPLATE", "INPUT_FILES"], num_args = 3.., required_unless_present_any = ["single", "multiple_by_type"])]
    multiple: Vec<PathBuf>,

    /// Multiple-by-type mode: group inputs by type attribute and use type-specific schemas/templates
    #[arg(long = "multiple-by-type", value_names = ["TYPE_ATTR_PATH", "TEMPLATE_DIR", "INPUT_FILES"], num_args = 3.., required_unless_present_any = ["single", "multiple"])]
    multiple_by_type: Vec<PathBuf>,
}

struct ReplaceFilter;

impl Filter for ReplaceFilter {
    fn filter(&self, value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
        let s = value
            .as_str()
            .ok_or_else(|| tera::Error::msg("Filter 'replace' received a non-string value"))?;

        let old = args
            .get("old")
            .and_then(|v| v.as_str())
            .ok_or_else(|| tera::Error::msg("Filter 'replace' requires 'old' argument"))?;

        let new = args
            .get("new")
            .and_then(|v| v.as_str())
            .ok_or_else(|| tera::Error::msg("Filter 'replace' requires 'new' argument"))?;

        let times = args.get("times").and_then(|v| v.as_u64());

        let result = if let Some(n) = times {
            let mut result = s.to_string();
            let mut count = 0;
            while count < n {
                if let Some(pos) = result.find(old) {
                    result.replace_range(pos..pos + old.len(), new);
                    count += 1;
                } else {
                    break;
                }
            }
            result
        } else {
            s.replace(old, new)
        };

        Ok(Value::String(result))
    }
}

struct ReplaceRegexFilter;

impl Filter for ReplaceRegexFilter {
    fn filter(&self, value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
        let s = value.as_str().ok_or_else(|| {
            tera::Error::msg("Filter 'replace_regex' received a non-string value")
        })?;

        let old = args
            .get("old")
            .and_then(|v| v.as_str())
            .ok_or_else(|| tera::Error::msg("Filter 'replace_regex' requires 'old' argument"))?;

        let new = args
            .get("new")
            .and_then(|v| v.as_str())
            .ok_or_else(|| tera::Error::msg("Filter 'replace_regex' requires 'new' argument"))?;

        let regex = Regex::new(old)
            .map_err(|e| tera::Error::msg(format!("Invalid regex pattern: {}", e)))?;

        let times = args.get("times").and_then(|v| v.as_u64());

        let result = if let Some(n) = times {
            let mut result = s.to_string();
            for _ in 0..n {
                if regex.is_match(&result) {
                    result = regex.replace(&result, new).to_string();
                } else {
                    break;
                }
            }
            result
        } else {
            regex.replace_all(s, new).to_string()
        };

        Ok(Value::String(result))
    }
}

struct StripFilter;

impl Filter for StripFilter {
    fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
        let s = value
            .as_str()
            .ok_or_else(|| tera::Error::msg("Filter 'strip' received a non-string value"))?;

        Ok(Value::String(s.trim().to_string()))
    }
}

thread_local! {
    static CONTEXT_HOLDER: RefCell<Option<Context>> = const { RefCell::new(None) };
}

#[allow(dead_code)]
struct IncludeFileFunction {
    base_path: Option<PathBuf>,
}

impl IncludeFileFunction {
    fn new(base_path: Option<PathBuf>) -> Self {
        Self { base_path }
    }
}

impl Function for IncludeFileFunction {
    fn call(&self, args: &HashMap<String, Value>) -> tera::Result<Value> {
        let path = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| tera::Error::msg("Function 'include_file' requires 'path' argument"))?;

        let full_path = if let Some(ref base) = self.base_path {
            base.join(path)
        } else {
            PathBuf::from(path)
        };

        if let Some(ref base) = self.base_path {
            let canonical_base = base.canonicalize().map_err(|e| {
                tera::Error::msg(format!(
                    "Failed to canonicalize base path '{}': {}",
                    base.display(),
                    e
                ))
            })?;

            let canonical_full = full_path.canonicalize().map_err(|e| {
                tera::Error::msg(format!(
                    "Failed to access file '{}': {}",
                    full_path.display(),
                    e
                ))
            })?;

            if !canonical_full.starts_with(&canonical_base) {
                return Err(tera::Error::msg(format!(
                    "Security error: Path '{}' is outside the allowed base directory",
                    path
                )));
            }
        }

        let absolute_full_path = full_path
            .canonicalize()
            .unwrap_or_else(|_| full_path.clone());
        eprintln!(
            "Loading template via include_file: {}",
            absolute_full_path.display()
        );

        let file_content = fs::read_to_string(&full_path).map_err(|e| {
            tera::Error::msg(format!(
                "Failed to read file '{}': {}",
                full_path.display(),
                e
            ))
        })?;

        let context = CONTEXT_HOLDER.with(|holder| {
            holder
                .borrow()
                .clone()
                .ok_or_else(|| tera::Error::msg("Context not available for include_file function"))
        })?;

        let mut tera = Tera::default();
        register_custom_filters_and_functions(&mut tera, self.base_path.clone());
        tera.add_raw_template("included_template", &file_content)
            .map_err(|e| {
                tera::Error::msg(format!(
                    "Failed to parse included template '{}': {}",
                    full_path.display(),
                    e
                ))
            })?;

        let rendered = tera.render("included_template", &context)?;

        Ok(Value::String(rendered))
    }
}

fn register_custom_filters_and_functions(tera: &mut Tera, base_path: Option<PathBuf>) {
    tera.register_filter("replace", ReplaceFilter);
    tera.register_filter("replace_regex", ReplaceRegexFilter);
    tera.register_filter("strip", StripFilter);
    tera.register_function("include_file", IncludeFileFunction::new(base_path));
}

#[cfg(test)]
fn render_template(template_str: &str, context: &tera::Context) -> Result<String> {
    let mut tera = Tera::default();
    register_custom_filters_and_functions(&mut tera, None);
    tera.add_raw_template("template", template_str)?;
    let rendered = tera.render("template", context)?;
    Ok(rendered)
}

fn handle_single_mode(
    schema_path: &Path,
    template_path: &Path,
    input_path: &Path,
    output: Option<&Path>,
) -> Result<()> {
    debug!("Running in single mode");
    debug!("Schema: {}", schema_path.display());
    debug!("Template: {}", template_path.display());
    debug!("Input: {}", input_path.display());

    if !schema_path.exists() {
        let message = format!(
            "Error: Schema file does not exist: {}",
            schema_path.display()
        );
        error!("{}", message);
        eprintln!("{}", message);
        exit(2);
    }

    if !template_path.exists() {
        let message = format!(
            "Error: Template file does not exist: {}",
            template_path.display()
        );
        error!("{}", message);
        eprintln!("{}", message);
        exit(2);
    }

    if !input_path.exists() {
        let message = format!("Error: Input file does not exist: {}", input_path.display());
        error!("{}", message);
        eprintln!("{}", message);
        exit(2);
    }

    debug!("Reading input YAML from: {}", input_path.display());
    let yaml_str = fs::read_to_string(input_path)?;
    let yaml_data: YamlValue = serde_yaml::from_str(&yaml_str)?;

    let json_value: JsonValue = serde_json::from_str(&serde_json::to_string(&yaml_data)?)?;

    debug!("Validating input file: {}", input_path.display());
    debug!("\tAgainst schema: {}", schema_path.display());
    let validation_result: Result<(), Vec<String>> = validate_json_schema(schema_path, &json_value);
    match validation_result {
        Ok(_) => {
            debug!("\tValidation successful.");
        }
        Err(errors) => {
            let message = format!(
                "Error: Validation failed for file: {}\nAgainst schema: {}",
                input_path.display(),
                schema_path.display()
            );
            error!("{}", message);
            eprintln!("{}", message);
            for error in &errors {
                let error_msg = format!("  - {}", error);
                error!("{}", error_msg);
                eprintln!("{}", error_msg);
            }
            exit(3);
        }
    }

    let template_extension = template_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let is_adoc = template_extension == "adoc";

    debug!("Loading template from: {}", template_path.display());
    let absolute_template_path = template_path
        .canonicalize()
        .unwrap_or_else(|_| template_path.to_path_buf());
    eprintln!("Loading template: {}", absolute_template_path.display());
    let template_str = fs::read_to_string(template_path)?;

    let mut tera = Tera::default();
    register_custom_filters_and_functions(
        &mut tera,
        template_path.parent().map(|p| p.to_path_buf()),
    );

    let template_name = if is_adoc {
        "template.adoc"
    } else {
        "template.j2"
    };
    tera.add_raw_template(template_name, &template_str)?;

    let mut context = Context::new();
    if let YamlValue::Mapping(map) = yaml_data {
        for (key, value) in map {
            if let Some(key_str) = key.as_str() {
                debug!("\tFound key: {}", key_str);
                let json_str = serde_json::to_string(&value)?;
                let json_value: JsonValue = serde_json::from_str(&json_str)?;
                context.insert(key_str, &json_value);
            }
        }
    }

    debug!("Rendering template...");
    CONTEXT_HOLDER.with(|holder| {
        *holder.borrow_mut() = Some(context.clone());
    });
    let rendered = tera.render(template_name, &context)?;

    if let Some(output_path) = output {
        debug!("Writing output to: {}", output_path.display());
        fs::write(output_path, &rendered)?;
        eprintln!(
            "Template rendered successfully to {}",
            output_path.display()
        );
    } else {
        println!("{}", rendered);
    }

    Ok(())
}

fn handle_multiple_mode(
    schema_path: &Path,
    template_path: &Path,
    input_paths: &[PathBuf],
    output: Option<&Path>,
) -> Result<()> {
    debug!("Running in multiple mode");
    debug!("Schema: {}", schema_path.display());
    debug!("Template: {}", template_path.display());
    debug!("Number of input files: {}", input_paths.len());

    if !schema_path.exists() {
        let message = format!(
            "Error: Schema file does not exist: {}",
            schema_path.display()
        );
        error!("{}", message);
        eprintln!("{}", message);
        exit(2);
    }

    if !template_path.exists() {
        let message = format!(
            "Error: Template file does not exist: {}",
            template_path.display()
        );
        error!("{}", message);
        eprintln!("{}", message);
        exit(2);
    }

    for input_path in input_paths {
        if !input_path.exists() {
            let message = format!("Error: Input file does not exist: {}", input_path.display());
            error!("{}", message);
            eprintln!("{}", message);
            exit(2);
        }
    }

    let mut sorted_input_paths = input_paths.to_vec();
    sorted_input_paths.sort();

    let template_extension = template_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let is_adoc = template_extension == "adoc";

    debug!("Loading template from: {}", template_path.display());
    let absolute_template_path = template_path
        .canonicalize()
        .unwrap_or_else(|_| template_path.to_path_buf());
    eprintln!("Loading template: {}", absolute_template_path.display());
    let template_str = fs::read_to_string(template_path)?;

    let mut tera = Tera::default();
    register_custom_filters_and_functions(
        &mut tera,
        template_path.parent().map(|p| p.to_path_buf()),
    );

    let template_name = if is_adoc {
        "template.adoc"
    } else {
        "template.j2"
    };
    tera.add_raw_template(template_name, &template_str)?;

    let mut rendered_outputs = Vec::new();

    for input_path in &sorted_input_paths {
        debug!("Processing input file: {}", input_path.display());
        debug!("Reading input YAML from: {}", input_path.display());
        let yaml_str = fs::read_to_string(input_path)?;
        let yaml_data: YamlValue = serde_yaml::from_str(&yaml_str)?;

        let json_value: JsonValue = serde_json::from_str(&serde_json::to_string(&yaml_data)?)?;

        debug!("Validating input file: {}", input_path.display());
        debug!("\tAgainst schema: {}", schema_path.display());
        let validation_result: Result<(), Vec<String>> =
            validate_json_schema(schema_path, &json_value);
        match validation_result {
            Ok(_) => {
                debug!("\tValidation successful.");
            }
            Err(errors) => {
                let message = format!(
                    "Error: Validation failed for file: {}\nAgainst schema: {}",
                    input_path.display(),
                    schema_path.display()
                );
                error!("{}", message);
                eprintln!("{}", message);
                for error in &errors {
                    let error_msg = format!("  - {}", error);
                    error!("{}", error_msg);
                    eprintln!("{}", error_msg);
                }
                exit(3);
            }
        }

        let mut context = Context::new();
        if let YamlValue::Mapping(map) = yaml_data {
            for (key, value) in map {
                if let Some(key_str) = key.as_str() {
                    debug!("\tFound key: {}", key_str);
                    let json_str = serde_json::to_string(&value)?;
                    let json_value: JsonValue = serde_json::from_str(&json_str)?;
                    context.insert(key_str, &json_value);
                }
            }
        }

        debug!("Rendering template for: {}", input_path.display());
        CONTEXT_HOLDER.with(|holder| {
            *holder.borrow_mut() = Some(context.clone());
        });
        let rendered = tera.render(template_name, &context)?;
        rendered_outputs.push(rendered);
    }

    let final_output = rendered_outputs.join("\n\n\n");

    if let Some(output_path) = output {
        debug!("Writing output to: {}", output_path.display());
        fs::write(output_path, &final_output)?;
        eprintln!(
            "Templates rendered successfully to {}",
            output_path.display()
        );
    } else {
        println!("{}", final_output);
    }

    Ok(())
}

fn extract_type_from_yaml(yaml_val: &YamlValue, type_attr_path: &str) -> Option<String> {
    let parts: Vec<&str> = type_attr_path.trim_start_matches('.').split('.').collect();
    let mut current = yaml_val;
    for part in parts {
        if let YamlValue::Mapping(map) = current {
            current = map.iter().find(|(k, _)| k.as_str() == Some(part))?.1;
        } else {
            return None;
        }
    }
    current.as_str().map(|s| s.to_string())
}

fn handle_multiple_by_type_mode(
    type_attr_path: &str,
    template_dir: &Path,
    input_paths: &[PathBuf],
    format: OutputFormat,
    output: Option<&Path>,
) -> Result<()> {
    debug!("Running in multiple-by-type mode");
    debug!("Type attribute path: {}", type_attr_path);
    debug!("Template directory: {}", template_dir.display());
    debug!("Number of input files: {}", input_paths.len());

    if !template_dir.exists() || !template_dir.is_dir() {
        let message = format!(
            "Error: Template directory does not exist or is not a directory: {}",
            template_dir.display()
        );
        error!("{}", message);
        eprintln!("{}", message);
        exit(2);
    }

    for input_path in input_paths {
        if !input_path.exists() {
            let message = format!("Error: Input file does not exist: {}", input_path.display());
            error!("{}", message);
            eprintln!("{}", message);
            exit(2);
        }
    }

    let mut sorted_input_paths = input_paths.to_vec();
    sorted_input_paths.sort();

    let mut file_types: HashMap<PathBuf, String> = HashMap::new();
    for file in &sorted_input_paths {
        debug!("Reading type from file: {}", file.display());
        let content = fs::read_to_string(file)?;
        let yaml_val: YamlValue = serde_yaml::from_str(&content)?;

        let type_str = extract_type_from_yaml(&yaml_val, type_attr_path).ok_or_else(|| {
            anyhow::anyhow!(
                "File {} does not have a '{}' field or field is not a string",
                file.display(),
                type_attr_path
            )
        })?;

        debug!("File {} has type: {}", file.display(), type_str);
        file_types.insert(file.clone(), type_str);
    }

    let template_suffix = format.template_suffix();
    let mut type_resources: HashMap<String, (PathBuf, PathBuf)> = HashMap::new();
    for type_name in file_types.values() {
        if !type_resources.contains_key(type_name) {
            let type_dir = template_dir.join(type_name);
            let schema_path = type_dir.join("schema.json");
            let template_filename = format!("template{}", template_suffix);
            let template_path = type_dir.join(&template_filename);

            if !schema_path.exists() {
                let message = format!(
                    "Error: Schema file does not exist: {}",
                    schema_path.display()
                );
                error!("{}", message);
                eprintln!("{}", message);
                exit(2);
            }
            if !template_path.exists() {
                let message = format!(
                    "Error: Template file does not exist: {}",
                    template_path.display()
                );
                error!("{}", message);
                eprintln!("{}", message);
                exit(2);
            }

            debug!(
                "Type '{}' uses schema: {} and template: {}",
                type_name,
                schema_path.display(),
                template_path.display()
            );
            type_resources.insert(type_name.clone(), (schema_path, template_path));
        }
    }

    let mut sorted_file_types: Vec<_> = file_types.iter().collect();
    sorted_file_types.sort_by_key(|(file, _)| *file);

    let mut files_by_type: HashMap<String, Vec<&PathBuf>> = HashMap::new();
    for (file, type_name) in sorted_file_types {
        files_by_type
            .entry(type_name.clone())
            .or_default()
            .push(file);
    }

    for files in files_by_type.values_mut() {
        files.sort();
    }

    let mut rendered_outputs = Vec::new();

    let mut sorted_types: Vec<_> = files_by_type.keys().collect();
    sorted_types.sort();

    for type_name in sorted_types {
        let files = &files_by_type[type_name];
        let (schema_path, template_path) = &type_resources[type_name];

        debug!("Processing files of type '{}'", type_name);

        let absolute_template_path = template_path
            .canonicalize()
            .unwrap_or_else(|_| template_path.clone());
        eprintln!(
            "Loading template for type '{}': {}",
            type_name,
            absolute_template_path.display()
        );
        let template_str = fs::read_to_string(template_path)?;
        let mut tera = Tera::default();
        register_custom_filters_and_functions(
            &mut tera,
            template_path.parent().map(|p| p.to_path_buf()),
        );

        let template_extension = template_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let is_adoc = template_extension == "adoc";
        let template_name = if is_adoc {
            "template.adoc"
        } else {
            "template.j2"
        };
        tera.add_raw_template(template_name, &template_str)?;

        for file in files {
            debug!("Validating file: {}", file.display());
            debug!("\tAgainst schema: {}", schema_path.display());

            let content = fs::read_to_string(file)?;
            let yaml_val: YamlValue = serde_yaml::from_str(&content)?;
            let json_value: JsonValue = serde_json::from_str(&serde_json::to_string(&yaml_val)?)?;

            let validation_result: Result<(), Vec<String>> =
                validate_json_schema(schema_path, &json_value);
            match validation_result {
                Ok(_) => {
                    debug!("\tValidation successful.");
                }
                Err(errors) => {
                    let message = format!(
                        "Error: Validation failed for file: {}\nAgainst schema: {}",
                        file.display(),
                        schema_path.display()
                    );
                    error!("{}", message);
                    eprintln!("{}", message);
                    for error in &errors {
                        let error_msg = format!("  - {}", error);
                        error!("{}", error_msg);
                        eprintln!("{}", error_msg);
                    }
                    exit(3);
                }
            }

            let mut context = Context::new();
            if let YamlValue::Mapping(map) = &yaml_val {
                for (key, value) in map {
                    if let Some(key_str) = key.as_str() {
                        debug!("\tFound key: {}", key_str);
                        let json_str = serde_json::to_string(value)?;
                        let json_value: JsonValue = serde_json::from_str(&json_str)?;
                        context.insert(key_str, &json_value);
                    }
                }
            }

            debug!("Rendering template for: {}", file.display());
            CONTEXT_HOLDER.with(|holder| {
                *holder.borrow_mut() = Some(context.clone());
            });
            let rendered = tera.render(template_name, &context)?;
            rendered_outputs.push(rendered);
        }
    }

    let final_output = rendered_outputs.join("\n\n\n");

    if let Some(output_path) = output {
        debug!("Writing output to: {}", output_path.display());
        fs::write(output_path, &final_output)?;
        eprintln!(
            "Templates rendered successfully to {}",
            output_path.display()
        );
    } else {
        println!("{}", final_output);
    }

    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting tpdg");

    let _raw_args: Vec<String> = std::env::args().collect();
    debug!("Raw arguments: {:?}", _raw_args);

    let args = Args::parse();
    debug!("Parsed arguments: {:?}", args);

    if !args.single.is_empty() {
        if args.single.len() != 3 {
            usage(
                "Single mode requires exactly 3 arguments: SCHEMA, TEMPLATE, INPUT",
                1,
            )
        }
        let schema_path = &args.single[0];
        let template_path = &args.single[1];
        let input_path = &args.single[2];
        return handle_single_mode(
            schema_path,
            template_path,
            input_path,
            args.output.as_deref(),
        );
    }

    if !args.multiple.is_empty() {
        if args.multiple.len() < 3 {
            usage(
                "Multiple mode requires at least 3 arguments: SCHEMA, TEMPLATE, INPUT_FILES...",
                1,
            )
        }
        let schema_path = &args.multiple[0];
        let template_path = &args.multiple[1];
        let input_paths = &args.multiple[2..];
        return handle_multiple_mode(
            schema_path,
            template_path,
            input_paths,
            args.output.as_deref(),
        );
    }

    if !args.multiple_by_type.is_empty() {
        if args.multiple_by_type.len() < 3 {
            usage(
                "Multiple-by-type mode requires at least 3 arguments: TYPE_ATTR_PATH, TEMPLATE_DIR, INPUT_FILES...",
                1,
            )
        }
        let type_attr_path = args.multiple_by_type[0]
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 in type attribute path"))?;
        let template_dir = &args.multiple_by_type[1];
        let input_paths = &args.multiple_by_type[2..];
        return handle_multiple_by_type_mode(
            type_attr_path,
            template_dir,
            input_paths,
            args.format,
            args.output.as_deref(),
        );
    }

    usage(
        "No valid mode specified. Use --single, --multiple, or --multiple-by-type",
        1,
    )
}

fn validate_json_schema(
    schema_path: &Path,
    payload: &serde_json::Value,
) -> Result<(), Vec<String>> {
    let schema_str = match fs::read_to_string(schema_path) {
        Ok(s) => s,
        Err(e) => {
            let error_msg = format!(
                "Failed to read schema file {}: {}",
                schema_path.display(),
                e
            );
            error!("{}", error_msg);
            return Err(vec![error_msg]);
        }
    };
    let schema_json = serde_json::from_str(&schema_str);
    let schema_json = match schema_json {
        Ok(s) => s,
        Err(e) => {
            let error_msg = format!(
                "Failed to parse schema file {}: {}",
                schema_path.display(),
                e
            );
            error!("{}", error_msg);
            return Err(vec![error_msg]);
        }
    };
    let schema_box = Box::new(schema_json);
    let schema_static: &'static JsonValue = Box::leak(schema_box);
    let compiled = match JSONSchema::compile(schema_static) {
        Ok(c) => c,
        Err(e) => {
            let error_msg = format!(
                "Failed to compile schema from file {}: {}",
                schema_path.display(),
                e
            );
            error!("{}", error_msg);
            return Err(vec![error_msg]);
        }
    };
    debug!("\tValidating payload against schema...");
    let validation_result = compiled.validate(payload);
    match validation_result {
        Ok(_) => {
            debug!("\tSchema validation: VALID");
            Ok(())
        }
        Err(errors) => {
            debug!("\tSchema validation: INVALID");
            let error_messages: Vec<String> = errors.into_iter().map(|e| e.to_string()).collect();
            Err(error_messages)
        }
    }
}

fn usage(message: &str, ret_code: i32) -> ! {
    error!("{}", message);
    eprintln!("Error: {}", message);
    exit(ret_code)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_output_format_template_suffix() {
        assert_eq!(OutputFormat::Markdown.template_suffix(), ".j2");
        assert_eq!(OutputFormat::Asciidoc.template_suffix(), "_asciidoc.adoc");
    }

    mod test_schema_validation {
        use super::*;
        use std::fs;
        use std::path::PathBuf;
        use tempfile::TempDir;

        fn create_temp_schema(dir: &TempDir, filename: &str, content: &str) -> PathBuf {
            let path = dir.path().join(filename);
            fs::write(&path, content).expect("Failed to write schema file");
            path
        }

        #[test]
        fn test_valid_test_case_schema_passes() {
            let temp_dir = TempDir::new().unwrap();
            let schema_json = r#"{
                "$schema": "http://json-schema.org/draft-04/schema#",
                "type": "object",
                "properties": {
                    "type": {"type": "string", "enum": ["test"]},
                    "requirement": {"type": "string"},
                    "item": {"type": "integer"},
                    "tc": {"type": "integer"},
                    "id": {"type": "string"},
                    "description": {"type": "string"}
                },
                "required": ["type", "requirement", "item", "tc", "id", "description"]
            }"#;
            let schema_path = create_temp_schema(&temp_dir, "schema.json", schema_json);

            let payload = serde_json::json!({
                "type": "test",
                "requirement": "XXX100",
                "item": 1,
                "tc": 4,
                "id": "TC-001",
                "description": "Test description"
            });

            let result = validate_json_schema(&schema_path, &payload);
            assert!(result.is_ok(), "Valid payload should pass validation");
        }

        #[test]
        fn test_invalid_test_case_schema_fails_with_error_message() {
            let temp_dir = TempDir::new().unwrap();
            let schema_json = r#"{
                "$schema": "http://json-schema.org/draft-04/schema#",
                "type": "object",
                "properties": {
                    "type": {"type": "string", "enum": ["test"]},
                    "requirement": {"type": "string"},
                    "item": {"type": "integer"},
                    "tc": {"type": "integer"}
                },
                "required": ["type", "requirement", "item", "tc"]
            }"#;
            let schema_path = create_temp_schema(&temp_dir, "schema.json", schema_json);

            let payload = serde_json::json!({
                "type": "test",
                "requirement": "XXX100",
                "item": 1
            });

            let result = validate_json_schema(&schema_path, &payload);
            assert!(
                result.is_err(),
                "Missing required field should fail validation"
            );
            let errors = result.unwrap_err();
            assert!(!errors.is_empty(), "Should have at least one error message");
            assert!(
                errors[0].contains("tc") || errors[0].contains("required"),
                "Error message should mention missing required field, got: {}",
                errors[0]
            );
        }
    }
}
