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
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    if path.ends_with("deps") {
        path.pop();
    }
    path.push("tpdg");
    path
}

#[test]
fn test_e2e_basic_yaml_rendering() {
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempdir().unwrap();
    let yaml_path = dir.path().join("data.yaml");
    let template_path = dir.path().join("template.tera");
    let output_path = dir.path().join("output.txt");

    let mut yaml_file = File::create(&yaml_path).unwrap();
    writeln!(yaml_file, "name: InstaTest\nage: 42").unwrap();

    let mut template_file = File::create(&template_path).unwrap();
    writeln!(template_file, "Name: {{{{ name }}}}\nAge: {{{{ age }}}}").unwrap();

    let schema_path = dir.path().join("schema.json");
    std::fs::write(&schema_path, "{}").unwrap();

    let status = Command::new(get_binary_path())
        .arg("--single")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(yaml_path.to_str().unwrap())
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .status()
        .expect("failed to run binary");
    assert!(status.success());

    assert!(output_path.exists(), "Output file was not created");
    let output = std::fs::read_to_string(&output_path).unwrap();
    assert_snapshot!("e2e_basic_yaml_rendering", normalize(&output));
}

#[test]
fn test_e2e_stdout_output() {
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempdir().unwrap();
    let yaml_path = dir.path().join("data.yaml");
    let template_path = dir.path().join("template.tera");

    let mut yaml_file = File::create(&yaml_path).unwrap();
    writeln!(yaml_file, "title: Test Report\nversion: \"1.0\"").unwrap();

    let mut template_file = File::create(&template_path).unwrap();
    writeln!(
        template_file,
        "# {{{{ title }}}}\nVersion: {{{{ version }}}}"
    )
    .unwrap();

    let schema_path = dir.path().join("schema.json");
    std::fs::write(&schema_path, "{}").unwrap();

    let output = Command::new(get_binary_path())
        .arg("--single")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(yaml_path.to_str().unwrap())
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

    let mut template_file = File::create(&template_path).unwrap();
    writeln!(template_file, "# {{{{ project }}}} v{{{{ version }}}}\n").unwrap();
    writeln!(template_file, "## Features").unwrap();
    writeln!(template_file, "{{% for feature in features %}}").unwrap();
    writeln!(template_file, "- {{{{ feature }}}}").unwrap();
    writeln!(template_file, "{{% endfor %}}\n").unwrap();
    writeln!(template_file, "Author: {{{{ metadata.author }}}}").unwrap();
    writeln!(template_file, "Date: {{{{ metadata.date }}}}").unwrap();

    let schema_path = dir.path().join("schema.json");
    std::fs::write(&schema_path, "{}").unwrap();

    let status = Command::new(get_binary_path())
        .arg("--single")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(yaml_path.to_str().unwrap())
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
    let template_path = dir.path().join("template.tera");
    let output_path = dir.path().join("output.txt");

    let mut template_file = File::create(&template_path).unwrap();
    writeln!(template_file, "Status: {{{{ status }}}}").unwrap();

    let schema_path = dir.path().join("schema.json");
    std::fs::write(&schema_path, "{}").unwrap();

    let tc_file1 = dir.path().join("tc_file1.yaml");
    std::fs::write(&tc_file1, "status: complete\n").unwrap();
    let tc_file2 = dir.path().join("tc_file2.yaml");
    std::fs::write(&tc_file2, "status: in-progress\n").unwrap();
    let tc_file3 = dir.path().join("tc_file3.yaml");
    std::fs::write(&tc_file3, "status: pending\n").unwrap();

    let status = Command::new(get_binary_path())
        .arg("--multiple")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
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

    File::create(&yaml_path).unwrap();

    let mut template_file = File::create(&template_path).unwrap();
    writeln!(template_file, "Static Content Only").unwrap();

    let schema_path = dir.path().join("schema.json");
    std::fs::write(&schema_path, "{}").unwrap();

    let status = Command::new(get_binary_path())
        .arg("--single")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(yaml_path.to_str().unwrap())
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

    let mut yaml_file = File::create(&yaml_path).unwrap();
    writeln!(yaml_file, "name: test user").unwrap();
    writeln!(yaml_file, "count: 42").unwrap();

    let mut template_file = File::create(&template_path).unwrap();
    writeln!(template_file, "Name: {{{{ name | upper }}}}").unwrap();
    writeln!(template_file, "Count: {{{{ count }}}}").unwrap();

    let schema_path = dir.path().join("schema.json");
    std::fs::write(&schema_path, "{}").unwrap();

    let status = Command::new(get_binary_path())
        .arg("--single")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(yaml_path.to_str().unwrap())
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
fn test_e2e_invalid_container_payload() {
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempfile::tempdir().unwrap();

    let schema_path = dir.path().join("schema.json");
    let schema = r#"{
        "type": "object",
        "required": ["name"],
        "properties": { "name": { "type": "string" } }
    }"#;
    std::fs::write(&schema_path, schema).unwrap();

    let template_path = dir.path().join("template.tera");
    std::fs::write(&template_path, "Name: {{{{ name }}}}").unwrap();

    let data_path = dir.path().join("data.yml");
    std::fs::write(&data_path, "age: 30\n").unwrap();

    let bin = get_binary_path();

    use std::fs::File;
    use std::os::unix::io::{FromRawFd, IntoRawFd};
    use std::os::unix::process::CommandExt;

    let fd3_file = tempfile::tempfile().expect("failed to create temp file for fd3");
    let fd3_raw = fd3_file.into_raw_fd();

    let mut cmd = std::process::Command::new(bin);
    cmd.arg("--single")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(data_path.to_str().unwrap());

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

    assert!(
        !output_with_fd3.status.success(),
        "binary should have failed schema validation but exited success"
    );
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

    let fixture_template_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/custom_filters_template.tera");
    std::fs::copy(&fixture_template_path, &template_path).unwrap();

    let schema_path = dir.path().join("schema.json");
    std::fs::write(&schema_path, "{}").unwrap();

    let status = Command::new(get_binary_path())
        .arg("--single")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(yaml_path.to_str().unwrap())
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

#[test]
fn test_e2e_single_mode() {
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempdir().unwrap();
    let schema_path = dir.path().join("schema.json");
    let template_path = dir.path().join("template.j2");
    let input_path = dir.path().join("input.yaml");
    let output_path = dir.path().join("output.txt");

    std::fs::write(&schema_path, "{}").unwrap();

    let mut template_file = File::create(&template_path).unwrap();
    writeln!(template_file, "# {{{{ title }}}}").unwrap();
    writeln!(template_file, "Value: {{{{ value }}}}").unwrap();

    let mut input_file = File::create(&input_path).unwrap();
    writeln!(input_file, "title: Single Mode Test").unwrap();
    writeln!(input_file, "value: 42").unwrap();

    let status = Command::new(get_binary_path())
        .arg("--single")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(input_path.to_str().unwrap())
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .status()
        .expect("failed to run binary");

    assert!(status.success(), "single mode should succeed");
    assert!(output_path.exists(), "output file was not created");

    let output = std::fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("# Single Mode Test"));
    assert!(output.contains("Value: 42"));

    assert_snapshot!("e2e_single_mode", normalize(&output));
}

#[test]
fn test_e2e_multiple_mode() {
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempdir().unwrap();
    let schema_path = dir.path().join("schema.json");
    let template_path = dir.path().join("template.j2");
    let output_path = dir.path().join("output.txt");

    std::fs::write(&schema_path, "{}").unwrap();

    let mut template_file = File::create(&template_path).unwrap();
    writeln!(template_file, "Item: {{{{ name }}}} - {{{{ count }}}}").unwrap();

    let input1_path = dir.path().join("input1.yaml");
    let mut input1_file = File::create(&input1_path).unwrap();
    writeln!(input1_file, "name: First").unwrap();
    writeln!(input1_file, "count: 10").unwrap();

    let input2_path = dir.path().join("input2.yaml");
    let mut input2_file = File::create(&input2_path).unwrap();
    writeln!(input2_file, "name: Second").unwrap();
    writeln!(input2_file, "count: 20").unwrap();

    let input3_path = dir.path().join("input3.yaml");
    let mut input3_file = File::create(&input3_path).unwrap();
    writeln!(input3_file, "name: Third").unwrap();
    writeln!(input3_file, "count: 30").unwrap();

    let status = Command::new(get_binary_path())
        .arg("--multiple")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(input1_path.to_str().unwrap())
        .arg(input2_path.to_str().unwrap())
        .arg(input3_path.to_str().unwrap())
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .status()
        .expect("failed to run binary");

    assert!(status.success(), "multiple mode should succeed");
    assert!(output_path.exists(), "output file was not created");

    let output = std::fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("Item: First - 10"));
    assert!(output.contains("Item: Second - 20"));
    assert!(output.contains("Item: Third - 30"));

    assert_snapshot!("e2e_multiple_mode", normalize(&output));
}

#[test]
fn test_e2e_multiple_by_type_mode() {
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempdir().unwrap();
    let template_dir = dir.path().join("templates");
    let output_path = dir.path().join("output.txt");

    let test_dir = template_dir.join("test");
    let review_dir = template_dir.join("review");
    std::fs::create_dir_all(&test_dir).unwrap();
    std::fs::create_dir_all(&review_dir).unwrap();

    std::fs::write(test_dir.join("schema.json"), "{}").unwrap();
    let mut test_template = File::create(test_dir.join("template.j2")).unwrap();
    writeln!(test_template, "TEST: {{{{ id }}}} - {{{{ description }}}}").unwrap();

    std::fs::write(review_dir.join("schema.json"), "{}").unwrap();
    let mut review_template = File::create(review_dir.join("template.j2")).unwrap();
    writeln!(review_template, "REVIEW: {{{{ id }}}} - {{{{ status }}}}").unwrap();

    let input1_path = dir.path().join("input1.yaml");
    let mut input1_file = File::create(&input1_path).unwrap();
    writeln!(input1_file, "type: test").unwrap();
    writeln!(input1_file, "id: T001").unwrap();
    writeln!(input1_file, "description: Test one").unwrap();

    let input2_path = dir.path().join("input2.yaml");
    let mut input2_file = File::create(&input2_path).unwrap();
    writeln!(input2_file, "type: review").unwrap();
    writeln!(input2_file, "id: R001").unwrap();
    writeln!(input2_file, "status: Approved").unwrap();

    let input3_path = dir.path().join("input3.yaml");
    let mut input3_file = File::create(&input3_path).unwrap();
    writeln!(input3_file, "type: test").unwrap();
    writeln!(input3_file, "id: T002").unwrap();
    writeln!(input3_file, "description: Test two").unwrap();

    let status = Command::new(get_binary_path())
        .arg("--multiple-by-type")
        .arg("type")
        .arg(template_dir.to_str().unwrap())
        .arg(input1_path.to_str().unwrap())
        .arg(input2_path.to_str().unwrap())
        .arg(input3_path.to_str().unwrap())
        .arg("--format")
        .arg("md")
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .status()
        .expect("failed to run binary");

    assert!(status.success(), "multiple-by-type mode should succeed");
    assert!(output_path.exists(), "output file was not created");

    let output = std::fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("REVIEW: R001 - Approved"));
    assert!(output.contains("TEST: T001 - Test one"));
    assert!(output.contains("TEST: T002 - Test two"));

    assert_snapshot!("e2e_multiple_by_type_mode", normalize(&output));
}

#[test]
fn test_e2e_dataset_gsma_multiple_by_type() {
    std::env::set_var("INSTA_UPDATE", "auto");
    let bin = get_binary_path();

    let vm_dir = "./data/verification_methods";
    let td = tempdir().unwrap();
    let report_path = td.path().join("report.md");

    let mut cmd = Command::new(bin);
    cmd.arg("--multiple-by-type")
        .arg("type")
        .arg(vm_dir)
        .arg("./data/test_case/filter_test_01_TC.yml")
        .arg("./data/test_case/filter_test_02_AN.yml")
        .arg("./data/test_case/filter_test_03_IN.yml")
        .arg("./data/test_case/filter_test_04_DM.yml")
        .arg("./data/test_case/gsma_4.4.2.2_TC.yml")
        .arg("./data/test_case/gsma_4.4.2.3_TC.yml")
        .arg("./data/test_case/gsma_4.4.2.4_AN.yml")
        .arg("./data/test_case/gsma_4.4.2.5_DM.yml")
        .arg("./data/test_case/gsma_4.4.2.6_IN.yml")
        .arg("--format")
        .arg("md")
        .arg("-o")
        .arg(report_path.as_os_str());

    let status = cmd.status().expect("failed to execute tpdg");
    assert!(status.success(), "binary exited with non-zero status");

    assert!(report_path.exists(), "report.md was not created");
    let metadata = std::fs::metadata(&report_path).expect("failed to stat report.md");
    assert!(metadata.len() > 0, "report.md is empty");

    let output = std::fs::read_to_string(&report_path).expect("failed to read generated report.md");
    assert_snapshot!("e2e_dataset_gsma_multiple_by_type", normalize(&output));
}

#[test]
fn test_e2e_input_data_multiple_by_type() {
    std::env::set_var("INSTA_UPDATE", "auto");
    let bin = get_binary_path();

    let vm_dir = "./data/input_data/verification_methods";
    let td = tempdir().unwrap();
    let report_path = td.path().join("report.md");

    let mut cmd = Command::new(bin);
    cmd.arg("--multiple-by-type")
        .arg("type")
        .arg(vm_dir)
        .arg("./data/input_data/test_case/TEST_PASSING_001.yml")
        .arg("./data/input_data/test_case/TEST_FAILING_002.yml")
        .arg("./data/input_data/test_case/gsma_4.4.2.2_TC.yml")
        .arg("./data/input_data/test_case/gsma_4.4.2.3_TC.yml")
        .arg("--format")
        .arg("md")
        .arg("-o")
        .arg(report_path.as_os_str());

    let status = cmd.status().expect("failed to execute tpdg");
    assert!(status.success(), "binary exited with non-zero status");

    assert!(report_path.exists(), "report.md was not created");
    let metadata = std::fs::metadata(&report_path).expect("failed to stat report.md");
    assert!(metadata.len() > 0, "report.md is empty");

    let output = std::fs::read_to_string(&report_path).expect("failed to read generated report.md");
    assert_snapshot!("e2e_input_data_multiple_by_type", normalize(&output));
}

#[test]
fn test_e2e_test_results_multiple_by_type_adoc() {
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempdir().unwrap();
    let output_path = dir.path().join("output.adoc");

    let vm_dir = "./data/input_data/verification_methods";
    let tc_passing = "data/input_data/test_results/RESULT_TEST_PASSING_001.yml";
    let tc_failing = "data/input_data/test_results/RESULT_TEST_FAILING_002.yml";

    let mut cmd = Command::new(get_binary_path());
    cmd.arg("--multiple-by-type")
        .arg("type")
        .arg(vm_dir)
        .arg(tc_passing)
        .arg(tc_failing)
        .arg("--format")
        .arg("adoc")
        .arg("-o")
        .arg(output_path.as_os_str());

    let status = cmd.status().expect("failed to run binary");
    assert!(status.success(), "test results rendering should succeed");
    assert!(
        output_path.exists(),
        "test results output file was not created"
    );

    let output = std::fs::read_to_string(&output_path).expect("failed to read output file");

    assert!(
        output.contains("TEST_PASSING_001") || output.contains("TEST_FAILING_002"),
        "output should contain test case IDs"
    );

    assert_snapshot!("e2e_test_results_multiple_by_type_adoc", normalize(&output));
}

#[test]
fn test_e2e_test_results_multiple_by_type_md() {
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempdir().unwrap();
    let output_path = dir.path().join("output.md");

    let vm_dir = "./data/input_data/verification_methods";
    let tc_passing = "data/input_data/test_results/RESULT_TEST_PASSING_001.yml";
    let tc_failing = "data/input_data/test_results/RESULT_TEST_FAILING_002.yml";

    let mut cmd = Command::new(get_binary_path());
    cmd.arg("--multiple-by-type")
        .arg("type")
        .arg(vm_dir)
        .arg(tc_passing)
        .arg(tc_failing)
        .arg("--format")
        .arg("md")
        .arg("-o")
        .arg(output_path.as_os_str());

    let status = cmd.status().expect("failed to run binary");
    assert!(status.success(), "test results rendering should succeed");
    assert!(
        output_path.exists(),
        "test results output file was not created"
    );

    let output = std::fs::read_to_string(&output_path).expect("failed to read output file");

    assert!(
        output.contains("TEST_PASSING_001") || output.contains("TEST_FAILING_002"),
        "output should contain test case IDs"
    );

    assert_snapshot!("e2e_test_results_multiple_by_type_md", normalize(&output));
}

#[test]
fn test_e2e_nested_include_file() {
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempdir().unwrap();
    let output_path = dir.path().join("output.md");

    let vm_dir = dir.path().join("vm");
    let test_vm_dir = vm_dir.join("test");
    std::fs::create_dir_all(&test_vm_dir).unwrap();

    let test_schema_path = test_vm_dir.join("schema.json");
    std::fs::write(&test_schema_path, "{}").unwrap();

    let nested_template_path = test_vm_dir.join("nested.j2");
    let mut nested_template = File::create(&nested_template_path).unwrap();
    writeln!(nested_template, "**Nested content for {{{{ name }}}}**").unwrap();
    writeln!(nested_template, "Value: {{{{ value }}}}").unwrap();

    let test_template_path = test_vm_dir.join("template.j2");
    let mut test_template = File::create(&test_template_path).unwrap();
    writeln!(test_template, "## Test Case: {{{{ id }}}}").unwrap();
    writeln!(test_template).unwrap();
    writeln!(test_template, "{{{{ include_file(path=\"nested.j2\") }}}}").unwrap();
    writeln!(test_template).unwrap();
    writeln!(test_template, "Status: {{{{ status }}}}").unwrap();

    let tc_file1 = dir.path().join("tc1.yml");
    let mut tc1 = File::create(&tc_file1).unwrap();
    writeln!(tc1, "type: test").unwrap();
    writeln!(tc1, "id: TC-001").unwrap();
    writeln!(tc1, "name: First Test").unwrap();
    writeln!(tc1, "value: 100").unwrap();
    writeln!(tc1, "status: PASS").unwrap();

    let tc_file2 = dir.path().join("tc2.yml");
    let mut tc2 = File::create(&tc_file2).unwrap();
    writeln!(tc2, "type: test").unwrap();
    writeln!(tc2, "id: TC-002").unwrap();
    writeln!(tc2, "name: Second Test").unwrap();
    writeln!(tc2, "value: 200").unwrap();
    writeln!(tc2, "status: FAIL").unwrap();

    let mut cmd = Command::new(get_binary_path());
    cmd.arg("--multiple-by-type")
        .arg("type")
        .arg(vm_dir.to_str().unwrap())
        .arg(tc_file1.to_str().unwrap())
        .arg(tc_file2.to_str().unwrap())
        .arg("--format")
        .arg("md")
        .arg("-o")
        .arg(output_path.as_os_str());

    let status = cmd.status().expect("failed to run binary");
    assert!(status.success(), "binary should have succeeded");
    assert!(output_path.exists(), "output file was not created");

    let output = std::fs::read_to_string(&output_path).unwrap();

    assert!(
        output.contains("## Test Case: TC-001"),
        "output should contain first test case"
    );
    assert!(
        output.contains("**Nested content for First Test**"),
        "output should contain nested template content with first test variable"
    );
    assert!(
        output.contains("Value: 100"),
        "output should contain value from first test case"
    );
    assert!(
        output.contains("Status: PASS"),
        "output should contain first test case status"
    );
    assert!(
        output.contains("## Test Case: TC-002"),
        "output should contain second test case"
    );
    assert!(
        output.contains("**Nested content for Second Test**"),
        "output should contain nested template content with second test variable"
    );
    assert!(
        output.contains("Value: 200"),
        "output should contain value from second test case"
    );
    assert!(
        output.contains("Status: FAIL"),
        "output should contain second test case status"
    );

    assert_snapshot!("e2e_nested_include_file", normalize(&output));
}

#[test]
fn test_e2e_nested_include_with_filters() {
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempdir().unwrap();
    let output_path = dir.path().join("output.md");

    let vm_dir = dir.path().join("vm");
    let test_vm_dir = vm_dir.join("test");
    std::fs::create_dir_all(&test_vm_dir).unwrap();

    let test_schema_path = test_vm_dir.join("schema.json");
    std::fs::write(&test_schema_path, "{}").unwrap();

    let nested_template_path = test_vm_dir.join("format.j2");
    let mut nested_template = File::create(&nested_template_path).unwrap();
    writeln!(
        nested_template,
        "Formatted: {{{{ text | strip | replace(old='test', new='TEST') }}}}"
    )
    .unwrap();
    writeln!(
        nested_template,
        "Regex: {{{{ text | replace_regex(old='\\\\d+', new='#') }}}}"
    )
    .unwrap();

    let test_template_path = test_vm_dir.join("template.j2");
    let mut test_template = File::create(&test_template_path).unwrap();
    writeln!(test_template, "### {{{{ id }}}}").unwrap();
    writeln!(test_template, "{{{{ include_file(path=\"format.j2\") }}}}").unwrap();

    let tc_file1 = dir.path().join("tc1.yml");
    let mut tc1 = File::create(&tc_file1).unwrap();
    writeln!(tc1, "type: test").unwrap();
    writeln!(tc1, "id: TC-001").unwrap();
    writeln!(tc1, "text: '  test 123 data 456  '").unwrap();

    let mut cmd = Command::new(get_binary_path());
    cmd.arg("--multiple-by-type")
        .arg("type")
        .arg(vm_dir.to_str().unwrap())
        .arg(tc_file1.to_str().unwrap())
        .arg("--format")
        .arg("md")
        .arg("-o")
        .arg(output_path.as_os_str());

    let status = cmd.status().expect("failed to run binary");
    assert!(status.success(), "binary should have succeeded");
    assert!(output_path.exists(), "output file was not created");

    let output = std::fs::read_to_string(&output_path).unwrap();

    assert!(
        output.contains("### TC-001"),
        "output should contain test case ID"
    );
    assert!(
        output.contains("Formatted: TEST 123 data 456"),
        "output should contain filtered text with strip and replace applied"
    );
    assert!(
        output.contains("Regex:   test 123 data 456") || output.contains("Regex:   test # data #"),
        "output should contain the text with regex replacement"
    );

    assert_snapshot!("e2e_nested_include_with_filters", normalize(&output));
}

#[test]
fn test_e2e_multiple_levels_nested_include() {
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempdir().unwrap();
    let output_path = dir.path().join("output.md");

    let vm_dir = dir.path().join("vm");
    let test_vm_dir = vm_dir.join("test");
    std::fs::create_dir_all(&test_vm_dir).unwrap();

    let test_schema_path = test_vm_dir.join("schema.json");
    std::fs::write(&test_schema_path, "{}").unwrap();

    let level3_template_path = test_vm_dir.join("level3.j2");
    let mut level3_template = File::create(&level3_template_path).unwrap();
    writeln!(level3_template, "Level3: {{{{ deep_value }}}}").unwrap();

    let level2_template_path = test_vm_dir.join("level2.j2");
    let mut level2_template = File::create(&level2_template_path).unwrap();
    writeln!(level2_template, "Level2: {{{{ mid_value }}}}").unwrap();
    writeln!(
        level2_template,
        "{{{{ include_file(path=\"level3.j2\") }}}}"
    )
    .unwrap();

    let level1_template_path = test_vm_dir.join("level1.j2");
    let mut level1_template = File::create(&level1_template_path).unwrap();
    writeln!(level1_template, "Level1: {{{{ top_value }}}}").unwrap();
    writeln!(
        level1_template,
        "{{{{ include_file(path=\"level2.j2\") }}}}"
    )
    .unwrap();

    let test_template_path = test_vm_dir.join("template.j2");
    let mut test_template = File::create(&test_template_path).unwrap();
    writeln!(test_template, "# {{{{ id }}}}").unwrap();
    writeln!(test_template, "{{{{ include_file(path=\"level1.j2\") }}}}").unwrap();

    let tc_file1 = dir.path().join("tc1.yml");
    let mut tc1 = File::create(&tc_file1).unwrap();
    writeln!(tc1, "type: test").unwrap();
    writeln!(tc1, "id: TC-NESTED").unwrap();
    writeln!(tc1, "top_value: TOP").unwrap();
    writeln!(tc1, "mid_value: MID").unwrap();
    writeln!(tc1, "deep_value: DEEP").unwrap();

    let mut cmd = Command::new(get_binary_path());
    cmd.arg("--multiple-by-type")
        .arg("type")
        .arg(vm_dir.to_str().unwrap())
        .arg(tc_file1.to_str().unwrap())
        .arg("--format")
        .arg("md")
        .arg("-o")
        .arg(output_path.as_os_str());

    let status = cmd.status().expect("failed to run binary");
    assert!(status.success(), "binary should have succeeded");
    assert!(output_path.exists(), "output file was not created");

    let output = std::fs::read_to_string(&output_path).unwrap();

    assert!(
        output.contains("# TC-NESTED"),
        "output should contain test case ID"
    );
    assert!(
        output.contains("Level1: TOP"),
        "output should contain level 1 content"
    );
    assert!(
        output.contains("Level2: MID"),
        "output should contain level 2 content"
    );
    assert!(
        output.contains("Level3: DEEP"),
        "output should contain level 3 content"
    );

    let level1_pos = output
        .find("Level1: TOP")
        .expect("Level1 should be present");
    let level2_pos = output
        .find("Level2: MID")
        .expect("Level2 should be present");
    let level3_pos = output
        .find("Level3: DEEP")
        .expect("Level3 should be present");

    assert!(
        level1_pos < level2_pos && level2_pos < level3_pos,
        "nested includes should maintain correct order"
    );

    assert_snapshot!("e2e_multiple_levels_nested_include", normalize(&output));
}

#[test]
fn test_e2e_invalid_schema_validation() {
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempfile::tempdir().unwrap();

    let schema_path = dir.path().join("schema.json");
    let schema = r#"{
        "type": "object",
        "required": ["name"],
        "properties": { "name": { "type": "string" } }
    }"#;
    std::fs::write(&schema_path, schema).unwrap();

    let template_path = dir.path().join("template.tera");
    std::fs::write(&template_path, "Name: {{{{ name }}}}").unwrap();

    let data_path = dir.path().join("data.yml");
    std::fs::write(&data_path, "age: 30\n").unwrap();

    let bin = get_binary_path();

    let output = Command::new(bin)
        .arg("--single")
        .arg(schema_path.to_str().unwrap())
        .arg(template_path.to_str().unwrap())
        .arg(data_path.to_str().unwrap())
        .output()
        .expect("failed to run binary");

    assert!(
        !output.status.success(),
        "binary should have failed schema validation but exited success"
    );
}

#[test]
fn test_e2e_multiple_by_type_invalid_type() {
    std::env::set_var("INSTA_UPDATE", "auto");

    let dir = tempdir().unwrap();
    let template_dir = dir.path().join("templates");
    let output_path = dir.path().join("output.txt");

    let test_dir = template_dir.join("test");
    std::fs::create_dir_all(&test_dir).unwrap();

    std::fs::write(test_dir.join("schema.json"), "{}").unwrap();
    let mut test_template = File::create(test_dir.join("template.j2")).unwrap();
    writeln!(test_template, "TEST: {{{{ id }}}}").unwrap();

    let input1_path = dir.path().join("input1.yaml");
    let mut input1_file = File::create(&input1_path).unwrap();
    writeln!(input1_file, "type: test").unwrap();
    writeln!(input1_file, "id: T001").unwrap();

    let input2_path = dir.path().join("input2.yaml");
    let mut input2_file = File::create(&input2_path).unwrap();
    writeln!(input2_file, "type: nonexistent").unwrap();
    writeln!(input2_file, "id: N001").unwrap();

    let output = Command::new(get_binary_path())
        .arg("--multiple-by-type")
        .arg("type")
        .arg(template_dir.to_str().unwrap())
        .arg(input1_path.to_str().unwrap())
        .arg(input2_path.to_str().unwrap())
        .arg("--format")
        .arg("md")
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .output()
        .expect("failed to run binary");

    assert!(
        !output.status.success(),
        "binary should fail when type directory does not exist"
    );
}
