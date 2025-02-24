Please analyze the provided file and extract file's key information to assist in file organization. Return the results in the following JSON format. If certain fields are not applicable or unavailable, return them with a null value. If a PDF file does not contain textual information, prioritize using OCR to extract the text. The extracted details should be useful for determining how the file should be categorized or stored:

```json
{
  "title": "Project Report for Tauri Integration",
  "version": "v1.0",
  "author": {
    "name": "Alice Chen",
    "organization": "Tauri Dev Team",
    "school_class": "",
    "id": "E12345",
    "semester": "2025spring",
    "team": "Integration Group"
  },
  "subject": "Software Development",
  "summary": "This report details the integration of Tauri within the Sortifile project...",
  "intended_use": "Reference material for project documentation and future development",
  "section_range": "Sections 1 to 5",
  "status": "Final",
  "topics": ["Integration Architecture", "Performance Benchmarks"],
  "tags": ["Tauri", "Rust", "Vue", "Sortifile"],
  "file_format": "DOCX",
  "language": "EN",
}
```

Ensure that the extracted summary, topics, and intended use provide a clear basis for categorizing or organizing the file. Return only the JSON object as output, formatted correctly for further processing.
