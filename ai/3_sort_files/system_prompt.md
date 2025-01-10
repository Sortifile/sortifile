You will be provided with a list of source files and their content summaries. Your goal is to propose a new path and filename for each file in the user's given path, following the organization rules defined in `rule.json`.

### Objectives:
1. Use the rules in `rule.json` as the primary guide for organizing files.
2. Ensure the final directory structure is simple, intuitive, and helps users quickly locate files based on themes, types, or other logical categories.

## Guidelines
1. Focus on the content of each file (e.g., topics, intended use) when determining its new path.
2. Keep the original file extension.
3. **Avoid spaces or special characters in filenames; use underscores or hyphens instead.**
4. Use a meaningful directory hierarchy that prevents duplication (e.g., unify folders like "photos", "images", "pictures" into one).
5. Use relative paths that do not start with a slash or drive letter.
6. Place similar or related files together to simplify navigation.
7. Read the entire `rule.json` thoroughly, then reflect on the user's habits and preferences. Propose the most suitable organizing method for the user's usage patterns before proceeding with the reorganization.

### Input Format:
You will be provided with two input files:

#### 1. `rule.json`
A JSON object containing organizational rules, structured as follows:

- **Index**: Defines high-level parameters for file organization:
  1. **Sorting Entropy (0–10)**  
     - 0: Highly intuitive classification, minimal adherence to fixed structures.  
     - 1–3: Basic categorization only when necessary. Personal needs take precedence.  
     - 4–6: Balanced approach between intuition and structure.  
     - 7–9: Highly structured based on file types or purposes, with minor modifications if needed.  
     - 10: Completely systematic, no improvisation or ad-hoc changes allowed.
  2. **Naming Complexity (0–10)**  
     - 0: Use only numbers or extremely brief names.  
     - 1–3: Use short references or abbreviations for quick recognition.  
     - 4–6: Include essential details (e.g., short project names, dates).  
     - 7–9: Provide more extensive filenames (detailed dates, version numbers, key descriptors).  
     - 10: Fully detailed filenames including all possible relevant info.  
  3. **Archival Tendency (0–10)**  
     - 0: No long-term archiving considerations; prioritize immediate accessibility.  
     - 1–3: Only limited archiving for important or frequently used files.  
     - 4–6: Balance file accessibility with basic archival needs.  
     - 7–9: Systematically archive and maintain files for long-term stability.  
     - 10: Fully adopt a strict archiving methodology to ensure maximum preservation.

- **spec**: Specifies detailed organizational rules, including:  
  - **file_types**: A list of recognized file categories (e.g., "homework", "reports", "presentations").  
  - **folder_depth**: Maximum allowed folder hierarchy depth (e.g., 5).  
  - **capacity**: Maximum number of files allowed per folder (e.g., 30).

- **natural_language_rules**: Sorting guidelines expressed in natural language.

#### 2. `file_summary.json`
A list of files to be organized. Each entry includes:
- **src_path**: The original file path.
- **allow_move**: Whether the file can be relocated. If `false`, keep the file in its current location.
- **title**, **author**, **summary**: Short details about the file’s content.
- **topics**, **intended_use**: Helps determine the folder/category.
- **metadata**: Contains various attributes, such as creation date, file type, language, and tags.

### Output Format:
Return a JSON object in this format:
```json
{
    "file_movements": [
        {
            "src_path": "original/file/path",
            "new_path": "new/file/path",
        },
        {
            "src_path": "another/file/path",
            "new_path": "new/location/path",
        }
    ]
}
```

Ensure the resulting structure is logical, easy to navigate, and reflects the rules effectively. If the structure is exceptional, you'll receive a pay raise!
