use insta::assert_snapshot;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
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

    // Create verification methods directory structure
    let vm_dir = dir.path().join("verification_methods");
    let test_type_dir = vm_dir.join("test");
    std::fs::create_dir_all(&test_type_dir).unwrap();
    let tc_schema_path = test_type_dir.join("schema.json");
    std::fs::write(&tc_schema_path, "{}").unwrap();
    let tc_template_path = test_type_dir.join("template.j2");
    std::fs::write(&tc_template_path, "").unwrap();

    // Create test case file with type field
    let tc_file_path = dir.path().join("tc_file.yaml");
    std::fs::write(&tc_file_path, "type: test\n").unwrap();

    let status = Command::new(get_binary_path())
        .arg("--container")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(yaml_path.to_str().unwrap())
        .arg("--test-case")
        .arg(vm_dir.to_str().unwrap())
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

    // Create verification methods directory structure
    let vm_dir = dir.path().join("verification_methods");
    let test_type_dir = vm_dir.join("test");
    std::fs::create_dir_all(&test_type_dir).unwrap();
    let tc_schema_path = test_type_dir.join("schema.json");
    std::fs::write(&tc_schema_path, "{}").unwrap();
    let tc_template_path = test_type_dir.join("template.j2");
    std::fs::write(&tc_template_path, "").unwrap();

    // Create test case file with type field
    let tc_file_path = dir.path().join("tc_file.yaml");
    std::fs::write(&tc_file_path, "type: test\n").unwrap();

    let output = Command::new(get_binary_path())
        .arg("--container")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(yaml_path.to_str().unwrap())
        .arg("--test-case")
        .arg(vm_dir.to_str().unwrap())
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

    // Create verification methods directory structure
    let vm_dir = dir.path().join("verification_methods");
    let test_type_dir = vm_dir.join("test");
    std::fs::create_dir_all(&test_type_dir).unwrap();
    let tc_schema_path = test_type_dir.join("schema.json");
    std::fs::write(&tc_schema_path, "{}").unwrap();
    let tc_template_path = test_type_dir.join("template.j2");
    std::fs::write(&tc_template_path, "").unwrap();

    // Create test case file with type field
    let tc_file_path = dir.path().join("tc_file.yaml");
    std::fs::write(&tc_file_path, "type: test\n").unwrap();

    let status = Command::new(get_binary_path())
        .arg("--container")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(yaml_path.to_str().unwrap())
        .arg("--test-case")
        .arg(vm_dir.to_str().unwrap())
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

    // Create verification methods directory structure
    let vm_dir = dir.path().join("verification_methods");
    let test_type_dir = vm_dir.join("test");
    std::fs::create_dir_all(&test_type_dir).unwrap();
    let tc_schema_path = test_type_dir.join("schema.json");
    std::fs::write(&tc_schema_path, "{}").unwrap();
    let tc_template_path = test_type_dir.join("template.j2");
    std::fs::write(&tc_template_path, "").unwrap();

    // Create test case files with type field
    let tc_file1 = dir.path().join("tc_file1.yaml");
    std::fs::write(&tc_file1, "type: test\n").unwrap();
    let tc_file2 = dir.path().join("tc_file2.yaml");
    std::fs::write(&tc_file2, "type: test\n").unwrap();
    let tc_file3 = dir.path().join("tc_file3.yaml");
    std::fs::write(&tc_file3, "type: test\n").unwrap();

    let status = Command::new(get_binary_path())
        .arg("--container")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(yaml_path.to_str().unwrap())
        .arg("--test-case")
        .arg(vm_dir.to_str().unwrap())
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

    // Create verification methods directory structure
    let vm_dir = dir.path().join("verification_methods");
    let test_type_dir = vm_dir.join("test");
    std::fs::create_dir_all(&test_type_dir).unwrap();
    let tc_schema_path = test_type_dir.join("schema.json");
    std::fs::write(&tc_schema_path, "{}").unwrap();
    let tc_template_path = test_type_dir.join("template.j2");
    std::fs::write(&tc_template_path, "").unwrap();

    // Create test case file with type field
    let tc_file_path = dir.path().join("tc_file.yaml");
    std::fs::write(&tc_file_path, "type: test\n").unwrap();

    let status = Command::new(get_binary_path())
        .arg("--container")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(yaml_path.to_str().unwrap())
        .arg("--test-case")
        .arg(vm_dir.to_str().unwrap())
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

    // Create verification methods directory structure
    let vm_dir = dir.path().join("verification_methods");
    let test_type_dir = vm_dir.join("test");
    std::fs::create_dir_all(&test_type_dir).unwrap();
    let tc_schema_path = test_type_dir.join("schema.json");
    std::fs::write(&tc_schema_path, "{}").unwrap();
    let tc_template_path = test_type_dir.join("template.j2");
    std::fs::write(&tc_template_path, "").unwrap();

    // Create test case file with type field
    let tc_file_path = dir.path().join("tc_file.yaml");
    std::fs::write(&tc_file_path, "type: test\n").unwrap();

    let status = Command::new(get_binary_path())
        .arg("--container")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(yaml_path.to_str().unwrap())
        .arg("--test-case")
        .arg(vm_dir.to_str().unwrap())
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
#[cfg(unix)]
fn test_e2e_invalid_container_payload_existing_lines() {
    // Ensure snapshots updates don't interfere with this negative test
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempfile::tempdir().unwrap();

    let schema_path = "data/container/schema.json";
    let template_path = "data/container/template.j2";
    let data_path = "data/container/data.yml";

    let vm_dir = "data/verification_methods";
    let tc_data_path = "data/test_case/invalid/invalid_payload.yml";

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
        .arg(vm_dir)
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
#[cfg(unix)]
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

    // Create verification methods directory structure
    let vm_dir = dir.path().join("verification_methods");
    let test_type_dir = vm_dir.join("test");
    std::fs::create_dir_all(&test_type_dir).unwrap();
    let tc_schema_path = test_type_dir.join("schema.json");
    std::fs::write(&tc_schema_path, "{}").unwrap();
    let tc_template_path = test_type_dir.join("template.j2");
    std::fs::write(&tc_template_path, "").unwrap();

    // Create test case file with type field
    let tc_file_path = dir.path().join("tc_file.yml");
    std::fs::write(&tc_file_path, "type: test\n").unwrap();

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
        .arg(vm_dir.to_str().unwrap())
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
    let container_schema = "./data/container/schema.json";
    let container_template = "./data/container/template.j2";
    let container_file = "./data/container/data.yml";

    // Verification methods directory
    let vm_dir = "./data/verification_methods";

    let tc_dir = std::path::Path::new("./data/test_case");
    let tc_files = sorted_test_case_files(tc_dir);

    // Build args vector
    let mut cmd = Command::new(bin);
    cmd.arg("--container")
        .arg(container_schema)
        .arg(container_template)
        .arg(container_file);
    cmd.arg("--test-case").arg(vm_dir);
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
    let container_schema = "./data/container/schema.json";
    let container_template = "./data/container/template.j2";
    let container_file = "./data/container/data.yml";

    // Verification methods directory
    let vm_dir = "./data/verification_methods";

    // Expand all .yml files under the test_case directory
    let tc_dir = std::path::Path::new("./data/test_case");
    let tc_files = sorted_test_case_files(tc_dir);

    // Build args vector and run
    let mut cmd = std::process::Command::new(bin_path);
    cmd.arg("--container")
        .arg(container_schema)
        .arg(container_template)
        .arg(container_file);
    cmd.arg("--test-case").arg(vm_dir);
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

fn sorted_test_case_files(tc_dir: &Path) -> Vec<String> {
    let mut tc_files: Vec<String> = Vec::new();
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
        "{}",
        format!(
            "no test-case yml files found in {}, cannot proceed",
            tc_dir.display()
        )
    );

    tc_files.sort();
    tc_files
}

#[test]
fn test_e2e_requirement_aggregation() {
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempdir().unwrap();
    let output_path = dir.path().join("output.adoc");

    let vm_dir = dir.path().join("vm");
    let result_vm_dir = vm_dir.join("result");
    std::fs::create_dir_all(&result_vm_dir).unwrap();

    let result_schema_path = result_vm_dir.join("schema.json");
    std::fs::write(&result_schema_path, "{}").unwrap();

    let result_template_path = result_vm_dir.join("template_asciidoc.adoc");
    std::fs::copy(
        "./data/test_results/result_template_asciidoc.adoc",
        &result_template_path,
    )
    .unwrap();

    let req_agg_template_path = vm_dir.join("requirement_aggregation_template.adoc");
    std::fs::copy(
        "./data/test_results/requirement_aggregation_template.adoc",
        &req_agg_template_path,
    )
    .unwrap();

    let container_schema = "data/test_results/container_schema.json";
    let container_template = "data/test_results/container_template_asciidoc.adoc";
    let container_data = "data/test_results/container_data.yml";

    let tc_files = vec![
        "data/test_results/sample_gsma_4.4.2.2_TC.yml",
        "data/test_results/sample_gsma_4.4.2.3_TC.yml",
        "data/test_results/sample_gsma_4.4.2.4_AN.yml",
        "data/test_results/sample_gsma_4.4.2.5_DM.yml",
        "data/test_results/sample_gsma_4.4.2.6_IN.yml",
    ];

    let mut cmd = Command::new(get_binary_path());
    cmd.arg("--container")
        .arg(container_schema)
        .arg(container_template)
        .arg(container_data);
    cmd.arg("--test-case").arg(vm_dir.to_str().unwrap());
    for tc_file in &tc_files {
        cmd.arg(tc_file);
    }
    cmd.arg("--format")
        .arg("asciidoc")
        .arg("-o")
        .arg(output_path.as_os_str());

    let status = cmd.status().expect("failed to run binary");
    assert!(status.success(), "binary should have succeeded");

    assert!(output_path.exists(), "output file was not created");

    let output = std::fs::read_to_string(&output_path).expect("failed to read output file");

    assert!(
        output.contains("== Requirements Summary"),
        "output should contain Requirements Summary section"
    );

    assert!(
        output.contains("requirements_with_detail:"),
        "output should contain requirements_with_detail YAML section"
    );

    assert!(
        output.contains("status_per_requirement:"),
        "output should contain status_per_requirement YAML section"
    );

    assert!(
        output.contains("requirements_by_status:"),
        "output should contain requirements_by_status YAML section"
    );

    assert!(
        output.contains("- requirement: XXX100"),
        "output should contain requirement XXX100"
    );

    assert!(
        output.contains("- requirement: XXX200"),
        "output should contain requirement XXX200"
    );

    assert!(
        output.contains("- requirement: XXX300"),
        "output should contain requirement XXX300"
    );

    assert!(
        output.contains("- requirement: XXX400"),
        "output should contain requirement XXX400"
    );

    assert!(
        output.contains("pass: true") && output.contains("pass: false"),
        "output should contain both pass: true and pass: false entries"
    );

    let req_with_detail_start = output
        .find("requirements_with_detail:")
        .expect("requirements_with_detail should be present");
    let status_per_req_start = output
        .find("status_per_requirement:")
        .expect("status_per_requirement should be present");
    let req_by_status_start = output
        .find("requirements_by_status:")
        .expect("requirements_by_status should be present");

    assert!(
        req_with_detail_start < status_per_req_start,
        "requirements_with_detail should come before status_per_requirement"
    );
    assert!(
        status_per_req_start < req_by_status_start,
        "status_per_requirement should come before requirements_by_status"
    );

    let pass_section_start =
        req_by_status_start + output[req_by_status_start..].find("pass:").unwrap();
    let fail_section_start =
        req_by_status_start + output[req_by_status_start..].find("fail:").unwrap();

    assert!(
        pass_section_start < fail_section_start,
        "pass section should come before fail section in requirements_by_status"
    );

    assert_snapshot!("e2e_requirement_aggregation", normalize(&output));
}

#[test]
fn test_e2e_custom_tera_filters() {
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempdir().unwrap();
    let yaml_path = dir.path().join("data.yaml");
    let template_path = dir.path().join("template.tera");
    let output_path = dir.path().join("output.txt");

    let mut yaml_file = File::create(&yaml_path).unwrap();
    writeln!(yaml_file, "text_with_whitespace: \"  hello world  \"").unwrap();
    writeln!(yaml_file, "text_with_patterns: \"foo bar foo baz foo\"").unwrap();
    writeln!(yaml_file, "text_with_regex: \"test123abc456def789\"").unwrap();
    writeln!(yaml_file, "multiline_text: |").unwrap();
    writeln!(yaml_file, "  Line with spaces  ").unwrap();
    writeln!(yaml_file, "  Another line  ").unwrap();
    writeln!(yaml_file, "complex_pattern: \"AAA-BBB-CCC AAA-DDD-EEE\"").unwrap();

    let mut template_file = File::create(&template_path).unwrap();
    writeln!(template_file, "# Custom Filter Tests").unwrap();
    writeln!(template_file).unwrap();
    writeln!(template_file, "## Strip Filter").unwrap();
    writeln!(template_file, "Original: '{{{{ text_with_whitespace }}}}'").unwrap();
    writeln!(
        template_file,
        "Stripped: '{{{{ text_with_whitespace | strip }}}}'"
    )
    .unwrap();
    writeln!(template_file).unwrap();
    writeln!(template_file, "## Replace Filter").unwrap();
    writeln!(template_file, "Original: {{{{ text_with_patterns }}}}").unwrap();
    writeln!(
        template_file,
        "Replace all 'foo' with 'bar': {{{{ text_with_patterns | replace(old=\"foo\", new=\"bar\") }}}}"
    )
    .unwrap();
    writeln!(
        template_file,
        "Replace first 'foo' with 'qux': {{{{ text_with_patterns | replace(old=\"foo\", new=\"qux\", times=1) }}}}"
    )
    .unwrap();
    writeln!(
        template_file,
        "Replace two 'foo' with 'xyz': {{{{ text_with_patterns | replace(old=\"foo\", new=\"xyz\", times=2) }}}}"
    )
    .unwrap();
    writeln!(template_file).unwrap();
    writeln!(template_file, "## Replace Regex Filter").unwrap();
    writeln!(template_file, "Original: {{{{ text_with_regex }}}}").unwrap();
    writeln!(
        template_file,
        "Remove all digits: {{{{ text_with_regex | replace_regex(old=\"[0-9]+\", new=\"\") }}}}"
    )
    .unwrap();
    writeln!(
        template_file,
        "Replace digits with '#': {{{{ text_with_regex | replace_regex(old=\"[0-9]+\", new=\"#\") }}}}"
    )
    .unwrap();
    writeln!(
        template_file,
        "Replace first digit sequence: {{{{ text_with_regex | replace_regex(old=\"[0-9]+\", new=\"NUM\", times=1) }}}}"
    )
    .unwrap();
    writeln!(
        template_file,
        "Replace letters with '*': {{{{ text_with_regex | replace_regex(old=\"[a-z]+\", new=\"*\") }}}}"
    )
    .unwrap();
    writeln!(template_file).unwrap();
    writeln!(template_file, "## Chained Filters").unwrap();
    writeln!(
        template_file,
        "Strip then replace: '{{{{ text_with_whitespace | strip | replace(old=\"hello\", new=\"goodbye\") }}}}'"
    )
    .unwrap();
    writeln!(
        template_file,
        "Replace then strip: '{{{{ text_with_whitespace | replace(old=\"world\", new=\"universe\") | strip }}}}'"
    )
    .unwrap();
    writeln!(
        template_file,
        "Complex chain: {{{{ complex_pattern | replace(old=\"AAA\", new=\"XXX\") | replace_regex(old=\"-[A-Z]+\", new=\"\") | strip }}}}"
    )
    .unwrap();
    writeln!(template_file).unwrap();
    writeln!(template_file, "## Multiline Strip").unwrap();
    writeln!(
        template_file,
        "Multiline stripped: '{{{{ multiline_text | strip }}}}'"
    )
    .unwrap();

    let schema_path = dir.path().join("schema.json");
    std::fs::write(&schema_path, "{}").unwrap();

    let vm_dir = dir.path().join("verification_methods");
    let test_type_dir = vm_dir.join("test");
    std::fs::create_dir_all(&test_type_dir).unwrap();
    let tc_schema_path = test_type_dir.join("schema.json");
    std::fs::write(&tc_schema_path, "{}").unwrap();
    let tc_template_path = test_type_dir.join("template.j2");
    std::fs::write(&tc_template_path, "").unwrap();

    let tc_file_path = dir.path().join("tc_file.yaml");
    std::fs::write(&tc_file_path, "type: test\n").unwrap();

    let status = Command::new(get_binary_path())
        .arg("--container")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(yaml_path.to_str().unwrap())
        .arg("--test-case")
        .arg(vm_dir.to_str().unwrap())
        .arg(tc_file_path.to_str().unwrap())
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .status()
        .expect("failed to run binary");
    assert!(status.success());

    assert!(output_path.exists(), "Output file was not created");

    let output = std::fs::read_to_string(&output_path).unwrap();

    assert!(
        output.contains("# Custom Filter Tests"),
        "output should contain title"
    );

    assert!(
        output.contains("Original: '  hello world  '"),
        "output should contain original text with whitespace"
    );
    assert!(
        output.contains("Stripped: 'hello world'"),
        "output should contain stripped text"
    );

    assert!(
        output.contains("Original: foo bar foo baz foo"),
        "output should contain original pattern text"
    );
    assert!(
        output.contains("Replace all 'foo' with 'bar': bar bar bar baz bar"),
        "output should show all occurrences replaced"
    );
    assert!(
        output.contains("Replace first 'foo' with 'qux': qux bar foo baz foo"),
        "output should show first occurrence replaced"
    );
    assert!(
        output.contains("Replace two 'foo' with 'xyz': xyz bar xyz baz foo"),
        "output should show two occurrences replaced"
    );

    assert!(
        output.contains("Original: test123abc456def789"),
        "output should contain original regex text"
    );
    assert!(
        output.contains("Remove all digits: testabcdef"),
        "output should show digits removed"
    );
    assert!(
        output.contains("Replace digits with '#': test#abc#def#"),
        "output should show digits replaced with #"
    );
    assert!(
        output.contains("Replace first digit sequence: testNUMabc456def789"),
        "output should show first digit sequence replaced"
    );
    assert!(
        output.contains("Replace letters with '*': *123*456*789"),
        "output should show letters replaced with asterisks"
    );

    assert!(
        output.contains("Strip then replace: 'goodbye world'"),
        "output should show chained strip then replace"
    );
    assert!(
        output.contains("Replace then strip: 'hello universe'"),
        "output should show chained replace then strip"
    );
    assert!(
        output.contains("Complex chain: XXX XXX"),
        "output should show complex filter chain"
    );

    assert!(
        output.contains("Multiline stripped:"),
        "output should contain multiline strip section"
    );

    assert_snapshot!("e2e_custom_tera_filters", normalize(&output));
}
