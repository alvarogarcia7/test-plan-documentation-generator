# StrictDoc Templates

This directory contains StrictDoc-format templates for various documentation standards and domains.

## What is StrictDoc?

[StrictDoc](https://github.com/strictdoc-project/strictdoc) is a software requirements specification and documentation tool that uses a text-based DSL (Domain-Specific Language). It provides:

- Requirements management with unique IDs and traceability
- Documentation generation in multiple formats (HTML, PDF, Excel)
- Version control friendly plain-text format
- Customizable grammars for domain-specific documentation
- Automatic traceability matrices and coverage reports

## Available Templates

### DO-178C (Aviation Software Certification)

Complete set of DO-178C compliant documentation templates for safety-critical aviation software certification.

**Location:** `DO-178C/`

**Documents Included:**
- PSAC (Plan for Software Aspects of Certification)
- SVP (Software Verification Plan)
- SRD (Software Requirements Data)
- SDD (Software Design Description)
- SVCP (Software Verification Cases and Procedures)
- SAS (Software Accomplishment Summary)

**Features:**
- Custom DO-178C grammar with PLAN, REQUIREMENT, TEST_CASE elements
- Level A software example (Flight Management System)
- Complete traceability between requirements and tests
- 85+ documented elements
- Realistic aviation examples

**Documentation:**
- [DO-178C/README.md](DO-178C/README.md) - Comprehensive overview
- [DO-178C/QUICK_START.md](DO-178C/QUICK_START.md) - Quick start guide
- [DO-178C/EXAMPLES.md](DO-178C/EXAMPLES.md) - Practical examples
- [DO-178C/INDEX.md](DO-178C/INDEX.md) - Complete index

**Quick Start:**
```bash
cd DO-178C
strictdoc export .
open output/html/index.html
```

## Installation

Install StrictDoc:

```bash
# Using pip
pip install strictdoc

# Verify installation
strictdoc --version
```

## Usage

### View Documentation

Generate HTML documentation:

```bash
cd <template-directory>
strictdoc export .
open output/html/index.html
```

### Generate PDF

Generate PDF (requires LaTeX):

```bash
strictdoc export --formats=pdf <document.sdoc>
```

### Generate Traceability Matrix

Generate Excel traceability matrix:

```bash
strictdoc export --formats=excel .
open output/excel/traceability.xlsx
```

## Template Structure

Each template directory contains:

- `*.sdoc` - StrictDoc document files
- `grammar.sgra` - Custom grammar definition (if applicable)
- `strictdoc.py` - Project configuration (Python-based)
- `README.md` - Template documentation
- Supporting documentation (QUICK_START, EXAMPLES, INDEX)

## Creating New Templates

To create a new template based on existing ones:

1. Copy an existing template directory
2. Customize the grammar (if needed)
3. Update document UIDs and content
4. Update strictdoc.py configuration
5. Generate and verify output

## Version Control

StrictDoc documents are plain text and work well with Git:

```bash
git add *.sdoc *.sgra *.py
git commit -m "Add new requirements"
git diff <file.sdoc>
```

## Export Formats

StrictDoc supports multiple export formats:

- **HTML**: Interactive documentation with search and navigation
- **PDF**: Formal documentation for certification/delivery
- **Excel**: Traceability matrices and coverage analysis
- **RST**: ReStructuredText for Sphinx integration

## Use Cases

### Requirements Management
Track software requirements with unique IDs, status, and verification methods.

### Certification Projects
Generate DO-178C, ISO 26262, or other safety-critical documentation.

### Technical Documentation
Create structured technical documentation with traceability.

### Design Documentation
Document software architecture and low-level design.

### Verification Documentation
Track test cases, procedures, and results with traceability to requirements.

## Benefits Over Traditional Tools

| Aspect | Traditional (Word/PDF/Excel) | StrictDoc |
|--------|----------------------------|-----------|
| Version Control | Binary files, hard to diff | Plain text, Git-friendly |
| Traceability | Manual cross-references | Automatic via UIDs |
| Search | Limited | Full-text + filtering |
| Export | Manual process | Automated multi-format |
| Collaboration | Sequential editing | Concurrent via Git |
| Validation | Manual checking | Schema validation |
| Cost | Expensive licenses | Free and open source |

## Integration with Other Tools

StrictDoc can integrate with:

- **Git/GitHub/GitLab**: Version control and CI/CD
- **DOORS**: Import/export via ReqIF
- **Jira**: Link requirements to issues
- **Jenkins**: Automated documentation generation
- **Sphinx**: Include in larger documentation projects

## Best Practices

1. **Use Consistent UIDs**: Follow a naming convention (e.g., REQ-XXX-###)
2. **Maintain Traceability**: Link requirements to tests and design
3. **Version Control**: Commit documents regularly
4. **Generate Often**: Regenerate HTML/PDF to verify changes
5. **Review Outputs**: Check generated documentation for formatting
6. **Automate**: Use scripts for batch exports
7. **Validate**: Run `strictdoc check` before commits

## Resources

### StrictDoc
- **GitHub**: https://github.com/strictdoc-project/strictdoc
- **Documentation**: https://strictdoc.readthedocs.io/
- **Examples**: https://github.com/strictdoc-project/strictdoc-examples

### Template Repositories
- **StrictDoc Templates**: https://github.com/strictdoc-project/strictdoc-templates
- **DO-178C Templates**: https://github.com/strictdoc-project/strictdoc-templates/tree/main/templates/DO-178C

### Standards
- **DO-178C**: Software Considerations in Airborne Systems and Equipment Certification
- **ISO 26262**: Road vehicles - Functional safety
- **IEC 61508**: Functional safety of electrical/electronic systems

## Contributing

Contributions to improve or extend these templates are welcome:

1. Fork the repository
2. Create a feature branch
3. Make improvements
4. Submit a pull request

## License

These templates are provided as examples. Customize and adapt them for your specific needs in compliance with applicable regulations and standards.

## Support

For questions and issues:

1. Review template documentation (README, QUICK_START, EXAMPLES)
2. Check StrictDoc documentation
3. Search existing issues on GitHub
4. Create a new issue with details

## Changelog

### v1.0 (2024-03-27)
- Initial release
- DO-178C templates with 6 complete documents
- Flight Management System example
- Comprehensive documentation and examples
- Custom grammar and project configuration
