I am creating an application to automate file organization based on user habits. You will receive:

1. An existing rule.json describing the current organizational rules and preferences, which the AI uses to automatically organize files.
2. A set of **file movement records** (`file_movements.json`), detailing how files were relocated.

Note that **when files are moved by the user, there is no explicit `reason` field provided**—the user’s motive is not directly stated. For system-initiated moves, a `reason` is recorded. Your task is to:

1. **Infer possible reasons** behind user-initiated moves by analyzing:
   - The file’s old (`src_path`) and new (`new_path`) locations.
   - The metadata in `summary` (e.g., topics, intended_use, tags).
   - Any observed patterns in folder structure or naming conventions.
2. **Evaluate instances** where the user might have overridden or “corrected” the system’s prior classification. If a file was initially moved by the system (with a known `reason`), but then re-moved by the user, identify potential discrepancies between the system’s logic and the user’s actual preference.
3. **Update and refine** the `rule.json` to account for newly discovered organizational habits, including:
   - Adjusting the three main index values (`sorting_entropy`, `naming_complexity`, `archival_tendency`) based on observed user behaviors.
   - Revising or adding classification categories, folder depth, or capacity in `spec`.
   - Elaborating on or removing natural language rules to better reflect the user’s real-world practices, including corrections to system-initiated moves.

### Objective

Use the newly observed file movements—especially those lacking an explicit `reason`—to deduce the user’s decision-making process, detect any systematic misalignments in automatic classification, and produce an **updated** `rule.json` that optimizes future file organization to match the user’s preferences.

### Input Data

1. **File Movement Records** (`file_movements.json`):  
   A list of objects, each containing:

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

2. **Existing File Organization Rules** (`rule.json`):
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

### Task Instructions

1. **Infer User Intent for User-Moved Files**

   - Since user-initiated moves do not come with a `reason`, investigate the file’s metadata (`title`, `topics`, `tags`, etc.) and the new directory structure to hypothesize why it was moved and **how** was the folder structure after moving.
   - Consider whether the move aligns with a known pattern (e.g., grouping by topic, date-based naming, project-based folders, etc.) or a newly observed preference.

2. **Identify Corrections to System Logic**

   - Look for files previously moved by the system (with an explicit `reason`) that were **later re-moved** by the user.
   - Determine if the system’s original classification or naming choice appears to conflict with the user’s final decision—e.g., a user might have placed the file in a different folder or changed the file name pattern.
   - Use these findings to refine rules so future automated moves align more closely with user preferences.

3. **Revise the Existing `rule.json`**

   - **Update Index Values**:

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

   - **Update `spec`**:
     - Populate quantifiable or specific details based on form responses, such as supported file types, folder depth, or capacity limits.
   - **Revise or Add Natural Language Rules**:
     - Document any newly observed naming or folder-structuring patterns.
     - Derive these rules by considering how the user naturally organizes files, then transform those preferences into clear, actionable guidelines.
     - Incorporate user preferences and general best practices (e.g., consistency, clarity, and searchability).
     - If the system’s classification was overridden, clarify how to avoid such missteps in the future (e.g., paying closer attention to topics or tags, using stricter date-based naming conventions).
     - You may add rules for specific scenarios (e.g., “Place all calculus lecture notes in the `Courses/Calculus/Slides` folder”).
     - Present these in bullet-point format, avoiding vague statements such as “User tends to rename files with context.” Instead, give concrete, directive-style rules.

### Output Format

generate the `rule.json` file in the specified format.

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

### Additional Considerations

- Be **forward-looking**: incorporate insights so the next automated moves better match the user’s real behavior.
- Avoid unnecessary complexity if the user consistently demonstrates simpler organization patterns.
- Ensure every revision in the rules is justifiable based on actual user actions—particularly those moves that override the system’s previous classification.
