import os
import json  # 用於格式化 JSON
from dotenv import load_dotenv
import google.generativeai as genai

# 載入環境變數
load_dotenv()
genai.configure(api_key=os.environ["GEMINI_API_KEY"])

# 建立模型配置
generation_config = {
    "temperature": 1,
    "top_p": 0.9,
    "top_k": 64,
    "max_output_tokens": 32768,
    "response_mime_type": "application/json",
}

# 讀取系統提示
system_prompt_path = "1_generate_rules/system_prompt.md"
with open(system_prompt_path, "r", encoding="utf-8") as file:
    content = file.read()

# 建立模型
model = genai.GenerativeModel(
    model_name="gemini-exp-1206",
    generation_config=generation_config,
    system_instruction=content,
)

# 上傳文件
form_respond = genai.upload_file(path="1_generate_rules/form_respond.json", mime_type="text/plain")
form_question = genai.upload_file(path="1_generate_rules/form_question.json", mime_type="text/plain")

# 啟動聊天會話
chat_session = model.start_chat(history=[])

# 發送消息並獲取回應
response = chat_session.send_message(["form_question: ", form_question, "\nform_respond: ", form_respond])

# 打印回應文字
print(response.text)

# 嘗試解析 JSON 並格式化後寫入檔案
output_path = "1_generate_rules/respond.json"
try:
    # 將回應轉換為 JSON 並格式化
    parsed_json = json.loads(response.text)
    with open(output_path, "w", encoding="utf-8") as outfile:
        json.dump(parsed_json, outfile, indent=4, ensure_ascii=False)  # indent=4 表示縮排4格
    print(f"格式化的 JSON 已寫入 {output_path}")
except json.JSONDecodeError:
    # 若不是有效的 JSON，直接寫入原始回應
    with open(output_path, "w", encoding="utf-8") as outfile:
        outfile.write(response.text)
    print(f"回應無法解析為 JSON，已原樣寫入 {output_path}")
