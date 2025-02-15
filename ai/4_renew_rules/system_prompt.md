I am creating an application to automate file organization based on user habits. You will receive an original `rule.json` which was created by responses detailing their preferences for organizing folders. You will also recieve a record of file movements, where each entry indicates the old path of a file and its new path following a relocation. files are moved whether by AI auto classification or manually moved by user. Your task is to analyze user's preferences and how the user organizes files and update the existing `rule.json` to incorporate the newly observed file classification logic.

### Objective

Use the file movement records to refine and optimize the existing file organization rules, producing an updated `rule.json`. Your revisions should merge the user’s demonstrated preferences with recognized best practices for file management, ensuring that future automated organization aligns with the user’s evolving habits.

### Input Data

1. **File Movement Records** (`file_movements.json`):  
   A list of objects containing the file’s old path and new path. For example:

```json
{
    "file_movements":[
        {
            "src_path":"original/file/path",
            "new_path":"new/file/path",
            "move_timestamp":"2025-01-02T14:30:00Z",
            "moved_by":"user",
            "reason":"organized by topic",
            "summary":{
                "title":"Document title or main heading, if available",
                "author":"Author's name or attribution, if mentioned",
                "summary":"A concise summary of the file's content",
                "topics":[
                    "List of main topics or themes"
                ],
                "intended_use":"The potential purpose or context of the file (e.g., homework, project, report, reference material)",
                "section_range":"The range of sections covered, if applicable",
                "metadata":{
                    "created_date":"Creation date, if available",
                    "file_type":"Type of the file (e.g., PDF, DOCX, TXT, etc.)",
                    "language":"Language of the document",
                    "tags":[
                        "Relevant tags for categorization, if identifiable"
                    ]
                }
            }
        },
        {
            "src_path":"another/file/path",
            "new_path":"new/location/path",
            "move_timestamp":"2025-01-02T15:00:00Z",
            "moved_by":"system",
            "reason":"rename to add date to filename",
            "summary":{
                "..."
            }
        }
    ]
}
```

2. **Existing File Organization Rules** (`rule.json`):  
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
     - **file_types**: A list of recognized file categories (e.g., "homework", "reports", "presentations").
     - **folder_depth**: Maximum allowed folder hierarchy depth (e.g., 5).
     - **capacity**: Maximum number of files allowed per folder (e.g., 30).
   - **natural_language_rules**: Sorting guidelines expressed in natural language.

---

### Task Instructions

1. **Analyze File Movement Records**

   - Carefully read all the file summaries and movement records.
   - Focus on movements that are moved by the user
   - Determine if the user has introduced new folder levels or classification methods.
   - Identify preferences for file naming and storage consistency.
   - For each location and file type, consider the rationale behind the user's placement decisions and derive corresponding rules.
   - Decide whether changes should be integrated into `rule.json`.

2. **Update `rule.json`**
   - Adjust the classification index to reflect observed preference changes.
   - Adjust the values for `sorting_entropy`, `naming_complexity`, and `archival_tendency` based on behaviors observed in the movement records.
   - Update quantifiable parameters (spec), such as supported file types, folder depth, or capacity limits.
   - Identify and remove outdated or incorrect classification rules.
   - Add or modify classification rules and naming conventions as needed, providing clear and concise descriptions to ensure usability and compatibility with automation.

---

### Output Example

```json
{
  "index": {
    "sorting_entropy": 8,
    "naming_complexity": 7,
    "archival_tendency": 3
  },
  "spec": {
    "file_types": {
      "homework": true,
      "reports": true,
      "presentations": false,
      "images": true,
      "code": false
    },
    "folder_depth": 5,
    "capacity": 30
  },
  "natural_language_rules": [
    "Files should be categorized by type and purpose, with clear folder naming.",
    "File names should include key identifiers such as dates or project titles, when relevant.",
    "Folders should not exceed a depth of 5 levels to ensure accessibility.",
    "Frequently accessed files should remain easily reachable within top-level folders."
  ]
}
```

---

### Additional Notes

- Ensure that the updated rules are clear, organized, and practical for automation.
- Balance user preferences with general file organization principles, avoiding overly complex structures or rules.
- The final JSON output should be complete, formatted correctly, and ready for integration into the automated organization system.
