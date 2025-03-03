# sec-scan

Privacy Risk Scanner for Local Documents

sec-scan is a powerful CLI tool for detecting personal information in documents and identifying
privacy risks. It helps prevent accidental leakage of sensitive information by scanning files for
personal data before they are shared or published.

![image](https://github.com/user-attachments/assets/e5823cfc-6886-4f67-b697-3adb8746b8b8)

Features

- Multi-format Support: Scans text files, PDFs, DOCX documents, and more
- Advanced Detection: Combines local LLM (via Ollama) with regex patterns for high accuracy detection
- Privacy Focused: All processing happens locally - no data is sent to external servers
- Flexible Output: JSON-formatted reports for easy analysis and integration
- Parallel Processing: Efficiently scans large volumes of files
- Configurable: Customizable API endpoints, models, and detection strategies

Installation

Prerequisites

- https://www.rust-lang.org/tools/install must be installed
- https://ollama.com/ should be installed and configured with the Deepseek Coder model (optional, can
  run in regex-only mode)

Building from Source

# Build the project

```bash
cargo build --release
```

# Install (optional)

```bash
cargo install --path .
```

Usage

```bash
sec-scan provides two main commands:
```

Scanning Directories

# Scan current directory

```bash
sec-scan scan
```

# Scan a specific directory

```bash
sec-scan scan /path/to/directory
```

# Save results to a file

```bash
sec-scan scan /path/to/directory --output results.json
```

# Skip API usage and use regex-only detection (faster but less accurate)

```bash
sec-scan scan /path/to/directory --no-api
```

# Disable PDF scanning

```bash
sec-scan scan /path/to/directory --pdf false
```

# Show verbose logs

```bash
sec-scan scan /path/to/directory --verbose
```

# Scan a single file

```bash
sec-scan scan-file /path/to/file.txt
```

# Save results to a file

```bash
sec-scan scan-file /path/to/file.pdf --output results.json
```

# Use regex-only detection

```bash
sec-scan scan-file /path/to/file.docx --no-api
```

```bash
sec-scan produces JSON-formatted output containing detected personal information:
```

```json
[
  {
    "file": "path/to/file1.txt",
    "personal*information": [
      {
        "type*": "email",
        "value": "test@example.com",
        "line": 5,
        "start": 10,
        "end": 25
      },
      {
        "type_": "phone_number",
        "value": "090-1234-5678",
        "line": 12,
        "start": 3,
        "end": 17
      }
    ]
  },
  {
    "file": "path/to/file2.pdf",
    "personal_information": []
  }
]
```

Field Descriptions

- file: Path to the scanned file
- personal_information: Array of detected personal information items
  - type\_: Type of personal information (email, phone_number, credit_card, address, name, etc.)
  - value: The detected personal information content
  - line: Line number where the information was found
  - start: Starting character position within the line
  - end: Ending character position within the line

Detectable Information

sec-scan can detect various types of personal information:

1. Email Addresses: Standard email formats
2. Phone Numbers: Various formats including Japanese phone numbers
3. Credit Card Numbers: Major credit card formats with Luhn algorithm validation
4. Addresses: When using LLM-based detection
5. Names: When using LLM-based detection
6. Other PII: Depending on the detection method used

Detection Methods

sec-scan supports two detection methods:

1. LLM-based Detection (default)Uses Ollama API with the Deepseek Coder model for high-accuracy
   detection
2. Regex-based Detection (--no-api option)Uses predefined regex patterns to detect email addresses,
   phone numbers, and credit card numbers; faster but less comprehensive than the LLM approach

Architecture

sec-scan is built on a robust architecture inspired by Domain-Driven Design and Clean Architecture:

- Domain Layer: Core business logic and interfaces
- Application Layer: Orchestration of use cases
- Infrastructure Layer: Implementation details (file system access, API clients, etc.)
- Interface Layer: CLI interface and user interaction

The system is designed with a plugin architecture that makes it easy to add new detectors and file
format extractors.

Limitations

- Does not scan image files (PNG, JPEG, etc.)
- Cannot process encrypted files
- Limited support for older document formats (.doc)
- Performance depends on the Ollama API response time when using LLM detection

Contributing

Bug reports and feature requests are welcome via GitHub issues. Pull requests are also welcome.
