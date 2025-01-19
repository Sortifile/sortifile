I am creating an application to automate file organization based on user habits. You will receive:

1. An **existing** `rule.json` describing the current organizational rules and preferences, which the AI uses to automatically organize files.
2. A set of **file movement records** (`file_movements.json`), detailing how files were relocated.

> **Important**: For **user-initiated moves**, there is **no `reason` field**—the user’s motive is not explicitly stated. For **system-initiated moves**, a `reason` is provided.

---

### Objective

Your task is to analyze these file movement records to determine how the user organizes files and make corresponding updates to the existing `rule.json`. Specifically:

1. **Infer User Intent (for User-Moved Files)**

   - Investigate files moved by the user (those without a `reason`) by examining `src_path`, `new_path`, and any relevant metadata in `summary`.
   - Look for patterns in folder structure, naming conventions, or metadata (e.g., topics, intended use) that suggest the user’s rationale.

2. **Identify Corrections to System Logic**

   - Find cases where a file was first moved by the system (with a recorded `reason`), then subsequently moved again by the user.
   - Determine if the system’s initial classification or naming convention conflicted with the user’s final choice. Use these findings to improve rules and reduce future mismatches.

3. **Refine the Existing `rule.json`**
   - Adjust the three index values (`sorting_entropy`, `naming_complexity`, `archival_tendency`) in light of the user’s demonstrated behaviors.
   - Update or add classification categories (`file_types`), revise `folder_depth` or `capacity` as needed, and **revise or add natural language rules** to reflect real-world user practices.

---

### Input Data

1. **File Movement Records** (`file_movements.json`)  
   A list of JSON objects with the following structure:

   ```jsonc
   {
       "src_path": "original/file/path",
       "new_path": "new/file/path",
       "move_timestamp": "2025-01-02T14:30:00Z",
       "moved_by": "system" | "user",
       "reason": "<reason if any>",
       "summary": {
           "title": "...",
           "author": "...",
           "summary": "...",
           "topics": [ "...", "..." ],
           "intended_use": "...",
           "section_range": "...",
           "metadata": {
               "created_date": "...",
               "file_type": "...",
               "language": "...",
               "tags": [ "..." ]
           }
       }
   }
   ```

   - **User-initiated moves**: `reason` may be absent or empty.
   - **System-initiated moves**: `reason` explains why the system moved the file.

2. **Existing File Organization Rules** (`rule.json`)
   ```jsonc
   {
     "index": {
       "sorting_entropy": <number 0~10>,
       "naming_complexity": <number 0~10>,
       "archival_tendency": <number 0~10>
     },
     "spec": {
       "file_types": {
         // recognized file types or categories
       },
       "folder_depth": <number>,
       "capacity": <number>
     },
     "natural_language_rules": [
       // human-readable guidelines or best practices
     ]
   }
   ```

---

### Revising Index Values

When you update `sorting_entropy`, `naming_complexity`, and `archival_tendency`, use these scales:

1. **Sorting Entropy (0–10)**

   - 0: Highly intuitive, minimal structure
   - 1–3: Basic categorization only when necessary
   - 4–6: Balanced between intuition and structure
   - 7–9: Highly structured with minor modifications
   - 10: Completely systematic, no ad-hoc changes

2. **Naming Complexity (0–10)**

   - 0: Extremely brief or numeric names
   - 1–3: Short references/abbreviations
   - 4–6: Essential details (e.g., short project names, dates)
   - 7–9: More extensive filenames (detailed dates, version numbers, key descriptors)
   - 10: Fully detailed filenames with all relevant info

3. **Archival Tendency (0–10)**
   - 0: No long-term archiving; prioritize immediate accessibility
   - 1–3: Limited archiving for important/frequent files
   - 4–6: Balanced accessibility vs. basic archival needs
   - 7–9: Systematic archiving for long-term stability
   - 10: Strict archiving methodology for maximum preservation

---

### Task Instructions

1. **Infer User Intent for User-Moved Files**

   - Since user-initiated moves lack a `reason`, examine paths, metadata (`title`, `topics`, `tags`, etc.), and the new folder structure.
   - Determine if these moves align with existing or newly emerging classification patterns.

2. **Identify Corrections to System Logic**

   - Locate files initially classified by the system (with a recorded `reason`), but later relocated by the user.
   - Evaluate whether the user’s changes indicate a different organizational preference than the system’s logic.

3. **Revise the Existing `rule.json`**
   - **Update Index Values** based on the user’s observed behavior.
   - **Update `spec`** to adjust file types, folder depth, and capacity constraints as necessary.
   - **Revise or Add Natural Language Rules** to reflect new naming/folder-structuring patterns or to correct system classification errors.
   - **Use Actionable Bullet Points** in your natural language rules. These rules can refers to particular files. (e.g., “Use `ProjectTitle_Version_Date` in the filename,” rather than a vague statement like “User tends to rename files.”)

---

### Output Format

Generate the final `rule.json` in valid JSON, with these sections:

```json
{
  "index": {
    "sorting_entropy": 6,
    "naming_complexity": 7,
    "archival_tendency": 4
  },
  "spec": {
    "file_types": {
      "homework": true,
      "reports": true,
      "images": true,
      "code": false,
      "references": true
    },
    "folder_depth": 4,
    "capacity": 50
  },
  "natural_language_rules": [
    "Include relevant dates in filenames to aid quick recognition.",
    "Group documents by major topics, and allow deeper nesting for archive materials.",
    "When system misclassifies a file, examine metadata tags before assigning a category."
  ]
}
```

---

### Additional Considerations

- Maintain a **forward-looking** approach: incorporate insights so future automated moves align more closely with the user’s preferences.
- If the user’s behavior suggests a simpler or more complex approach than the current rules, adjust accordingly.
- Every rule change should be justifiable by observed file movements—especially cases where the user overrides system classification.
