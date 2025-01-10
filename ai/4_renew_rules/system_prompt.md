You will be provided with a record of file movements, where each entry indicates the old path of a file and its new path following a manual relocation. Based on these records, analyze how the user organizes files and update the existing `rule.json` to incorporate the newly observed file classification logic.

### Objective
Use the file movement records to refine and optimize the existing file organization rules, producing an updated `rule.json`. Your revisions should merge the user’s demonstrated preferences with recognized best practices for file management, ensuring that future automated organization aligns with the user’s evolving habits.

### Input Data
1. **File Movement Records** (`file_movements.json`):  
    A list of objects containing the file’s old path and new path. For example:
    ```json
    {
        "file_movements": [
            {
                "src_path": "original/file/path",
                "dst_path": "new/file/path",
                "move_timestamp": "2025-01-02T14:30:00Z",
                "moved_by": "user",
                "reason": "organized by topic"
            },
            {
                "src_path": "another/file/path",
                "dst_path": "new/location/path",
                "move_timestamp": "2025-01-02T15:00:00Z",
                "moved_by": "system",
                "reason": "automated categorization"
            }
        ]
    }
    ```
2. **Existing File Organization Rules** (`rule.json`):  
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

---

### Task Instructions

1. **Analyze File Movement Records**  
    - Determine if the user has introduced new folder levels or classification methods.  
    - Identify preferences for file naming and storage consistency.  
    - For each location and file type, consider the rationale behind the user's placement decisions and derive corresponding rules.  
    - Decide whether changes should be integrated into `rule.json`.

2. **Update `rule.json`**  
    - Adjust the classification index to reflect observed preference changes. 
    - Adjust the values for `sorting_entropy`, `naming_complexity`, and `archival_tendency` based on behaviors observed in the movement records.
 
    - Update quantifiable parameters (spec), such as supported file types, folder depth, or capacity limits.  
    - Provide clear, concise descriptions of new classification logic and naming conventions to ensure usability and automation compatibility.
---

### Output Example
```json
{
    "index": {
        "sorting_entropy": 6,
        "naming_complexity": 8,
        "archival_tendency": 4
    },
    "spec": {
        "file_types": ["documents", "images", "videos", "archives"],
        "folder_depth": 6,
        "capacity": 40
    },
    "natural_language_rules": [
        "Documents should be organized by year and category.",
        "Images should be grouped by event or subject, with subfolders for personal and travel-related photos.",
        "Archive files (e.g., zip, tar) should be stored in a dedicated 'archives' folder organized by year."
    ]
}
```
---

### Additional Notes

- Ensure that the updated rules are clear, organized, and practical for automation.  
- Balance user preferences with general file organization principles, avoiding overly complex structures or rules.  
- The final JSON output should be complete, formatted correctly, and ready for integration into the automated organization system.