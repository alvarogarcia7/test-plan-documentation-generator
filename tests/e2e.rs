use insta::assert_snapshot;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tempfile::tempdir;

fn normalize(s: &str) -> String {
    let mut out = s.replace("\r\n", "\n");
    while out.contains("\n\n") {
        out = out.replace("\n\n", "\n");
    }
    out.trim_end().to_string()
}

fn get_binary_path() -> PathBuf {
    // Get the path to the compiled binary in target/debug
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    if path.ends_with("deps") {
        path.pop();
    }
    path.push("test-plan-doc-gen");
    path
}

#[test]
fn test_e2e_basic_yaml_rendering() {
    std::env::set_var("INSTA_UPDATE", "auto");

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
    let output = std::fs::read_to_string(&output_path).unwrap();
    assert_snapshot!("e2e_basic_yaml_rendering", normalize(&output));
}

#[test]
fn test_e2e_stdout_output() {
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempdir().unwrap();
    let yaml_path = dir.path().join("data.yaml");
    let template_path = dir.path().join("template.tera");

    // Write YAML file
    let mut yaml_file = File::create(&yaml_path).unwrap();
    // Write version as a string to preserve formatting when rendered
    writeln!(yaml_file, "title: Test Report\nversion: \"1.0\"").unwrap();

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
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    assert_snapshot!("e2e_stdout_output", normalize(&stdout));
}

#[test]
fn test_e2e_complex_yaml_structure() {
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempdir().unwrap();
    let yaml_path = dir.path().join("data.yaml");
    let template_path = dir.path().join("template.tera");
    let output_path = dir.path().join("output.txt");

    // Write complex YAML with nested structures
    let mut yaml_file = File::create(&yaml_path).unwrap();
    writeln!(yaml_file, "project: TestSuite").unwrap();
    writeln!(yaml_file, "version: \"2.0\"").unwrap();
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

    let output = std::fs::read_to_string(&output_path).unwrap();
    assert_snapshot!("e2e_complex_yaml_structure", normalize(&output));
}

#[test]
fn test_e2e_multiple_test_case_files() {
    std::env::set_var("INSTA_UPDATE", "auto");

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

    let output = std::fs::read_to_string(&output_path).unwrap();
    assert_snapshot!("e2e_multiple_test_case_files", normalize(&output));
}

#[test]
fn test_e2e_empty_yaml() {
    std::env::set_var("INSTA_UPDATE", "auto");

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

    let output = std::fs::read_to_string(&output_path).unwrap();
    assert_snapshot!("e2e_empty_yaml", normalize(&output));
}

#[test]
fn test_e2e_template_with_filters() {
    std::env::set_var("INSTA_UPDATE", "auto");

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

    let output = std::fs::read_to_string(&output_path).unwrap();
    assert_snapshot!("e2e_template_with_filters", normalize(&output));
}

#[test]
fn test_e2e_invalid_container_payload_existing_lines() {
    // Ensure snapshots updates don't interfere with this negative test
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempfile::tempdir().unwrap();

    let schema_path = "data/dataset_4_GSMA/container/schema.json";
    let template_path = "data/dataset_4_GSMA/container/template.j2";
    let data_path = "data/dataset_4_GSMA/container/data.yml";

    let tc_schema_path = "data/dataset_4_GSMA/test_case/schema.json";
    let tc_template_path = "data/dataset_4_GSMA/test_case/template.j2";
    let tc_data_path = "data/dataset_4_GSMA/test_case/invalid_payload.yml";

    // Build and run the binary
    let bin = get_binary_path();

    let output_path = dir.path().join("report.md");

    // Capture file descriptor 3 output
    use std::fs::File;
    use std::os::unix::io::{FromRawFd, IntoRawFd};
    use std::os::unix::process::CommandExt;

    let fd3_file = tempfile::tempfile().expect("failed to create temp file for fd3");
    let fd3_raw = fd3_file.into_raw_fd();

    let mut cmd = std::process::Command::new(bin);
    cmd.arg("--container")
        .arg(schema_path)
        .arg(template_path)
        .arg(data_path)
        .arg("--test-case")
        .arg(tc_schema_path)
        .arg(tc_template_path)
        .arg(tc_data_path)
        .arg("-o")
        .arg(output_path);

    unsafe {
        cmd.pre_exec(move || {
            let temp = File::from_raw_fd(fd3_raw);
            std::mem::forget(temp);
            Ok(())
        });
    }

    let output_with_fd3 = cmd.output().expect("failed to run binary with fd3");
    println!("{:?}", output_with_fd3.stderr);
    println!("{:?}", output_with_fd3.stdout);

    // let mut fd3_output = String::new();
    // let mut fd3_file_read = unsafe { File::from_raw_fd(fd3_raw) };
    // use std::io::{Read, Seek, SeekFrom};
    // fd3_file_read
    //     .seek(SeekFrom::Start(0))
    //     .expect("failed to seek fd3");
    // fd3_file_read
    //     .read_to_string(&mut fd3_output)
    //     .expect("failed to read fd3");
    //
    // println!("File descriptor 3 output:");
    // println!("{}", String::from_utf8_lossy(fd3_output.as_bytes()));

    // We expect the program to fail validation and exit with non-zero
    assert!(
        !output_with_fd3.status.success(),
        "binary should have failed schema validation but exited success"
    );
}

#[test]
fn test_e2e_invalid_container_payload() {
    // Ensure snapshots updates don't interfere with this negative test
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempfile::tempdir().unwrap();

    // Write a schema that requires the property `name`
    let schema_path = dir.path().join("schema.json");
    let schema = r#"{
        "type": "object",
        "required": ["name"],
        "properties": { "name": { "type": "string" } }
    }"#;
    std::fs::write(&schema_path, schema).unwrap();

    // Write a container template (not important for validation)
    let template_path = dir.path().join("template.tera");
    std::fs::write(&template_path, "Name: {{{{ name }}}}").unwrap();

    // Write an invalid YAML file (missing `name`)
    let data_path = dir.path().join("data.yml");
    std::fs::write(&data_path, "age: 30\n").unwrap();

    // Create minimal dummy test-case args (schema, template, file)
    let tc_schema_path = dir.path().join("tc_schema.json");
    std::fs::write(&tc_schema_path, "{}").unwrap();
    let tc_template_path = dir.path().join("tc_template.tera");
    std::fs::write(&tc_template_path, "").unwrap();
    let tc_file_path = dir.path().join("tc_file.yml");
    std::fs::write(&tc_file_path, "{}").unwrap();

    // Build and run the binary
    let bin = get_binary_path();

    // Capture file descriptor 3 output
    use std::fs::File;
    use std::os::unix::io::{FromRawFd, IntoRawFd};
    use std::os::unix::process::CommandExt;

    let fd3_file = tempfile::tempfile().expect("failed to create temp file for fd3");
    let fd3_raw = fd3_file.into_raw_fd();

    let mut cmd = std::process::Command::new(bin);
    cmd.arg("--container")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(data_path.to_str().unwrap())
        .arg("--test-case")
        .arg(tc_schema_path.to_str().unwrap())
        .arg(tc_template_path.to_str().unwrap())
        .arg(tc_file_path.to_str().unwrap());

    unsafe {
        cmd.pre_exec(move || {
            let temp = File::from_raw_fd(fd3_raw);
            std::mem::forget(temp);
            Ok(())
        });
    }

    let output_with_fd3 = cmd.output().expect("failed to run binary with fd3");
    println!("{:?}", output_with_fd3.stderr);
    println!("{:?}", output_with_fd3.stdout);

    let mut fd3_output = String::new();
    let mut fd3_file_read = unsafe { File::from_raw_fd(fd3_raw) };
    use std::io::{Read, Seek, SeekFrom};
    fd3_file_read
        .seek(SeekFrom::Start(0))
        .expect("failed to seek fd3");
    fd3_file_read
        .read_to_string(&mut fd3_output)
        .expect("failed to read fd3");

    println!("File descriptor 3 output:");
    println!("{}", fd3_output);

    // We expect the program to fail validation and exit with non-zero
    assert!(
        !output_with_fd3.status.success(),
        "binary should have failed schema validation but exited success"
    );
}

#[test]
fn test_e2e_dataset_4_gsma() {
    std::env::set_var("INSTA_UPDATE", "auto");
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
    assert_snapshot!("e2e_dataset_4_gsma", normalize(&output));
}

#[test]
fn test_e2e_dataset_4_gsma_target_debug() {
    std::env::set_var("INSTA_UPDATE", "auto");
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
    assert!(
        status.success(),
        "binary exited with non-zero status: {:?}",
        status.code()
    );

    // Some binaries write in their working dir; check both locations.
    if !report_path.exists() {
        let alt = bin_path
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("report.md");
        if alt.exists() {
            std::fs::rename(&alt, &report_path).expect("failed to move report.md from binary dir");
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
    assert_snapshot!("e2e_dataset_4_gsma_target_debug", normalize(&output));
}
