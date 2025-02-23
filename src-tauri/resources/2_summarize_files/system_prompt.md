Please analyze the provided files and extract every file's key information to assist in file organization. Return the results in the following JSON format. If certain fields are not applicable or unavailable, return them with a null value. If a PDF file does not contain textual information, prioritize using OCR to extract the text. The extracted details should be useful for determining how the file should be categorized or stored:

```json
[
  {
    "title":"Document title or main heading, if available",
    "author":"Author's name or attribution, if mentioned",
    "summary":"A concise summary of the file's content, including key points or highlights",
    "topics":[
      "List of main topics or themes"
    ],
    "intended_use":"The potential purpose or context of the file (e.g., homework, project, report, reference material)",
    "section_range":"The range of sections covered, if applicable",
    "metadata":{
      "created_date":"Creation date, if available",
      "last_modified_date":"Last modification date, if applicable"
      "file_type":"Type of the file (e.g., PDF, DOCX, TXT, etc.)",
      "language":"Language of the document",
      "tags":[
        "Relevant tags for categorization, if identifiable"
      ],
    }
  },
  {
    "..."
  }
]
```

Ensure that the extracted summary, topics, and intended use provide a clear basis for categorizing or organizing the file. Return only the JSON object as output, formatted correctly for further processing.
