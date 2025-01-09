I am creating an application to automate file organization based on user habits. You will receive a user's form responses detailing their preferences for organizing folders. Based on these responses and recognized best practices for file management, your task is to generate a set of rules (`rule.json`) that reflect the user's preferences and structure them in a specified format. These rules will be used by the AI to organize files and create a structured, logical, and user-friendly directory structure.

Please use the data below to generate a `rule.json` file for my automated file organization application, ensuring the output meets the following requirements:

1. **Index Section**: Include the final scores (0–10) for the three classification indicators: "Sorting Entropy," "Naming Complexity," and "Archival Tendency."
2. **Spec Section**: Summarize the user's responses from `form_question.json` and `form_respond.json` to extract quantifiable or specific settings (e.g., file types, folder depth).
3. **Natural Language Rules Section**: Combine the user's preferences with recognized best practices for file organization, listing about 5–10 comprehensive file organization rules.

---

### Input Data

1. **`form_question.json`**  
   - Contains the questionnaire's questions, options, and the corresponding `rule.json` field mapping (`rule_mapping`).  
   - Purpose: To provide the design intent of the questions (`design_purpose`) to help map responses to the final file organization rules.

2. **`form_respond.json`**  
   - Contains the user's responses to the questionnaire.  
   - Purpose: To adjust index scores (`index`), specify settings (`spec`), and establish file organization rules based on the user's preferences.

---

### Three Classification Index Score Range

Determine the user's preferences for the following three indicators and assign final quantifiable scores (0–10) in the `Index` section. Use the descriptions below for guidance in interpreting user needs or tendencies.

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
---

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

1. **Index**  
   - Assign reasonable scores based on user responses.  
2. **Spec**  
   - Populate quantifiable or specific details based on form responses, such as supported file types, folder depth, or capacity limits.  
3. **Natural Language Rules**  
   - Include at least 5–10 rules.  
   - Derive these rules by thinking about how the user might categorize files and summarize those preferences into clear guidelines.  
   - Incorporate user preferences and general best practices (e.g., consistency, clarity, searchability).

---

### Additional Notes

1. **Output JSON only**: Avoid extra strings or additional explanations.  
2. **Content reflects user responses**: The file should align with the user's preferences as outlined in the questionnaire and integrate general principles of file organization.

