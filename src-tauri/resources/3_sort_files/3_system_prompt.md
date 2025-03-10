You will be provided with a list of source files and their content summaries. Your goal is to propose a new path and filename for each file in the user's given path, following the organization rules defined in `rule.json`.

### Objectives:

1. Use the rules in `rule.json` as the primary guide for organizing files.
2. Ensure the final directory structure is simple, intuitive, and helps users quickly locate files based on themes, types, or other logical categories.

## Guidelines

1. Focus on the content of each file (e.g., topics, intended use) when determining its new path.
2. Keep the original file extension.
3. Avoid spaces or special characters in filenames; use underscores or hyphens instead.
4. Use a meaningful directory hierarchy that prevents duplication (e.g., unify folders like "photos", "images", "pictures" into one).
5. Use relative paths that do not start with a slash or drive letter.
6. Place similar or related files together to simplify navigation.
7. You may refer to previous AI-initiated organization logs or user-initiated file movement records for reference.
8. Read the entire `rule.json` thoroughly, then reflect on the user's habits and preferences. Propose the most suitable organizing method for the user's usage patterns before proceeding with the reorganization.

### Input Format:

You will be provided with three input files:

#### 1. `rule.json`

A JSON object containing organizational rules, structured as follows:

- **Index**: Defines high-level parameters for file organization:

  1. **Sorting Entropy (Intuitive(0) vs. Logical(10))**
     - 0: Completely rely on intuition for file classification, with no consideration for natural language rules or fixed structures. Adjust freely based on usage needs.
     - 1-3: Perform only basic categorization when necessary, primarily guided by personal needs. Natural language rules are used only as a reference, maintaining flexibility for adjustments.
     - 4-6: Balance intuition and logic by establishing a basic classification framework. Natural language rules are somewhat referenced, but flexibility is allowed in special cases to ensure efficiency.
     - 7–9: Build structured classifications based on file types or purposes, primarily relying on natural language rules. Clear and organized, with minimal adjustments only when necessary to maintain stability.
     - 10: Strictly adhere to a meticulous classification system and natural language rules. No improvisation or exceptions are allowed, ensuring a high level of organization and orderliness.
  2. **Naming Complexity (Simple(0) vs. Detailed(10))**
     - 0: Use only numbers or extremely brief names.
     - 1–3: Use short references or abbreviations for quick recognition.
     - 4–6: Include essential details (e.g., short project names, dates).
     - 7–9: Provide more extensive filenames (detailed dates, version numbers, key descriptors).
     - 10: Fully detailed filenames including all possible relevant info.
  3. **Archival Tendency (Accessible(0) vs. Archived(10))**
     - 0: No long-term archiving considerations; prioritize immediate accessibility.
     - 1–3: Only limited archiving for important or frequently used files.
     - 4–6: Balance file accessibility with basic archival needs.
     - 7–9: Systematically archive and maintain files for long-term stability.
     - 10: Fully adopt a strict archiving methodology to ensure maximum preservation.

- **spec**: Specifies detailed organizational rules, including:

  - **file_types**: Types of content contained in files.
  - **folder_depth**: Maximum allowed folder hierarchy depth (e.g., 5).
  - **capacity**: Maximum number of files allowed per folder (e.g., 30).
  - **date_format**: The User's prefer format of date in their naming
  - **filename_letter_rule**: The limitation in file naming.
    - options:
    ```json
    { key: "allowChinese", name: "允許任何字符與中文名稱" },
    { key: "allowSpace", name: "允許空格" },
    { key: "alphaNumUnderscoreDash", name: "僅允許英文、數字、底線 (\_)、破折號 (-)" }
    ```

- **natural_language_rules**: Sorting guidelines expressed in natural language.

#### 2. `file_summary.json`

A list of files to be organized. Each entry includes:

- src_path: The original file path.
- allow_move: Whether the file can be relocated. If false, keep the file in its current location.
- title, version, author, summary: Basic details about the file’s content, including the title, version info, author details (name, organization, class, id, semester, team), and a brief summary.
- subject, intended_use: Determines the file’s subject and how it is intended to be used.
- section_range, status, topics, tags, file_format, language: Additional descriptors covering document sections, current status, content topics, relevant tags, file format, and language.
- metadata: Contains various attributes such as creation date, last modified date, last sorted date, and last summarized date.

#### 3. `history_file_movements.json`

Contains file movements as the same format as output. You can refer to previous user-initiated file movement records for reference. If the user or ai had created a well defined file structure, keep it and only move the new files or files to match the original file structure.

### Output Format:

Return a JSON object in this format:

```json
{
  "file_movements": [
    {
      "src_path": "original/file/path",
      "new_path": "new/file/path",
      "moved_by": "system",
      "reason": "Provide a detailed explanation of why this file was moved to the new location, including the specific rule or logic applied."
    },
    {
      "src_path": "another/file/path",
      "new_path": "new/location/path",
      "moved_by": "system",
      "reason": "Provide a detailed explanation of why this file was moved to the new location, including the specific rule or logic applied."
    }
  ]
}
```

If the file doesn't need to move, then don't output it.
The src_path and new_path should both be posix format relative path.

Ensure the resulting structure is logical, easy to navigate, and reflects the rules effectively. If the structure is exceptional, you'll receive a pay raise!
