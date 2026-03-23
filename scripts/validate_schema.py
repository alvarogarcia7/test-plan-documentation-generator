#!/usr/bin/env python3
"""
Validate a YAML file against a JSON schema.
"""
import sys
import json
import yaml

def main():
    if len(sys.argv) != 3:
        print("Usage: validate_schema.py <schema.json> <data.yml>", file=sys.stderr)
        sys.exit(1)
    
    schema_file = sys.argv[1]
    data_file = sys.argv[2]
    
    try:
        import jsonschema
    except ImportError:
        print("Error: jsonschema module not found. Install with: pip3 install jsonschema", file=sys.stderr)
        sys.exit(1)
    
    try:
        # Load JSON schema
        with open(schema_file, 'r') as f:
            schema = json.load(f)
        
        # Load YAML data
        with open(data_file, 'r') as f:
            data = yaml.safe_load(f)
        
        # Validate
        jsonschema.validate(instance=data, schema=schema)
        
        print(f"✓ Validation successful: {data_file} conforms to {schema_file}")
        sys.exit(0)
        
    except jsonschema.exceptions.ValidationError as e:
        print(f"✗ Validation failed for {data_file}:", file=sys.stderr)
        print(f"  Error: {e.message}", file=sys.stderr)
        if e.path:
            print(f"  Path: {'.'.join(str(p) for p in e.path)}", file=sys.stderr)
        sys.exit(1)
        
    except FileNotFoundError as e:
        print(f"✗ File not found: {e}", file=sys.stderr)
        sys.exit(1)
        
    except json.JSONDecodeError as e:
        print(f"✗ Invalid JSON in schema file: {e}", file=sys.stderr)
        sys.exit(1)
        
    except yaml.YAMLError as e:
        print(f"✗ Invalid YAML in data file: {e}", file=sys.stderr)
        sys.exit(1)
        
    except Exception as e:
        print(f"✗ Unexpected error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
