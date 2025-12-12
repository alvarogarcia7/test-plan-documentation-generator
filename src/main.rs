use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tera::Tera;

#[derive(Parser, Debug)]
#[command(name = "test-plan-doc-gen")]
#[command(about = "A CLI tool to render templates for test plan documentation", long_about = None)]
struct Args {
    /// Path to the template file
    #[arg(short, long)]
    template: PathBuf,

    /// Output file path
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// JSON data file for template variables
    #[arg(short, long)]
    data: Option<PathBuf>,
}

#[cfg(test)]
fn render_template(template_str: &str, context: &tera::Context) -> Result<String> {
    let mut tera = Tera::default();
    tera.add_raw_template("template", template_str)?;
    let rendered = tera.render("template", context)?;
    Ok(rendered)
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    // Create Tera instance and add template
    let template_str = std::fs::read_to_string(&args.template)?;
    let mut tera = Tera::default();
    tera.add_raw_template("template", &template_str)?;

    // Load data if provided
    let mut context = tera::Context::new();
    if let Some(data_path) = args.data {
        let data_str = std::fs::read_to_string(&data_path)?;
        let json_value: serde_json::Value = serde_json::from_str(&data_str)?;
        if let serde_json::Value::Object(map) = json_value {
            for (key, value) in map {
                context.insert(key, &value);
            }
        }
    }

    // Render template
    let rendered = tera.render("template", &context)?;

    // Output result
    if let Some(output_path) = args.output {
        std::fs::write(&output_path, &rendered)?;
        println!("Template rendered successfully to {}", output_path.display());
    } else {
        println!("{}", rendered);
    }

    Ok(())
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
}
