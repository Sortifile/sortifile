Please analyze the provided files and extract every file's key information to assist in file organization. Return the results in the following JSON format. If certain fields are not applicable or unavailable, return them with a null value. If a PDF file does not contain textual information, prioritize using OCR to extract the text. The extracted details should be useful for determining how the file should be categorized or stored:

```json
[
  {
    "title": "Document title or main heading, if available",
    "version": "Version number of the document, e.g., v1.0",
    "author": {
      "name": "作者姓名，如 John Doe",
      "organization": "作者隸屬機構或單位，若無則留空",
      "school_class": "學校/科系/年級/班級名稱，如『台大資工一』，若無則留空",
      "id": "學號、座號或員工編號，若無則留空",
      "semester": "學期資訊，如 113-2、2025spring 等，若無則留空",
      "team": "小組或專案組名稱，如『專題A組』，若無則留空"
    },
    "subject": "文件主要科目或領域，如 Calculus、History",
    "summary": "文件內容的精簡摘要或重點說明",
    "intended_use": "The potential purpose or context of the file (e.g., homework, project, report, reference material)",
    "section_range": "The range of sections covered, if applicable",
    "status": "文件狀態，如 '草稿'、'最終版'",
    "topics": ["文件中的主要主題或議題，如 ['微分','極限']"],
    "tags": ["用於分類或搜索的關鍵字，如 ['重要','參考資料']"],
    "file_format": "原始檔案格式，如 PDF、DOCX、TXT",
    "language": "文件語言，如 EN、ZH-TW",
    "metadata": {
      "created_date": "文件建立日期，如 '2025-02-20'",
      "last_modified_date": "最後修改日期，如 '2025-02-25'"
    }
  },
  {
    "..."
  }
]
```

Ensure that the extracted summary, topics, and intended use provide a clear basis for categorizing or organizing the file. Return only the JSON object as output, formatted correctly for further processing.
