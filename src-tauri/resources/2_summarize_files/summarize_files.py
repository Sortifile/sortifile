from pathlib import Path
import sys
import mimetypes
import json
import time


# 將專案根目錄添加到 sys.path
project_root = Path(__file__).resolve().parent.parent
sys.path.append(str(project_root))

from utils import *

# 預設的生成配置
DEFAULT_GENERATION_CONFIG = {
    "temperature": 1,
    "top_p": 0.9,
    "top_k": 64,
    "max_output_tokens": 32768,
    "response_mime_type": "application/json",
}

def process_files_in_directory(directory):
    """
    遞迴遍歷目錄並根據檔案類型執行不同的處理。

    Args:
        directory (str): 要遍歷的目錄路徑。

    Returns:
        list: 包含檔案相對路徑及其副檔名的列表。
    """
    base_path = Path(directory)
    file_list = []
    for file_path in base_path.rglob("*"):  # 遞迴遍歷
        if file_path.is_file() and not file_path.name.startswith("."):  # 確保是檔案
            relative_path = file_path.relative_to(base_path)  # 取得相對路徑
            file_extension = file_path.suffix.lower()  # 取得副檔名（小寫）
            
            file_list.append({"path": str(relative_path), "extension": file_extension})

    return file_list

def summarize_files(
    system_prompt_path="2_summarize_files/system_prompt.md",
    user_prompt_path="2_summarize_files/user_prompt.txt",
    folder_path="2_summarize_files/sample",
    rule_path="2_summarize_files/rule.json",
    output_path="2_summarize_files/respond.json",
    model_name="gemini-exp-1206",
    generation_config=None,
):
    """
    對目錄中的檔案進行摘要處理。

    Args:
        system_prompt_path (str): 系統提示檔案的路徑。
        user_prompt_path (str): 用戶提示檔案的路徑。
        folder_path (str): 要處理的目錄路徑。
        rule_path (str): 規則檔案的路徑。
        output_path (str): 輸出結果的檔案路徑。
        model_name (str): 使用的生成模型名稱。
        generation_config (dict, optional): 模型生成的配置選項。

    Returns:
        None
    """
    generation_config = generation_config or DEFAULT_GENERATION_CONFIG

    load_environment()

    with open(user_prompt_path, "r", encoding="utf-8") as file:
        user_prompt = file.read()

    model = configure_generation_model(system_prompt_path, model_name, generation_config)
    file_list = process_files_in_directory(folder_path)
    response_list = []

    for files in file_list:
        path = files["path"]
        extension = files["extension"]
        print(f"Processing {path} with extension {extension}")
        files["uploaded_files"] = upload_files(folder_path + "/" + path, mime_type=mimetypes.types_map.get(extension, "application/octet-stream"))
        message_components = [user_prompt, "\nfile_summary: ", files["uploaded_files"][0]]
        
        # 嘗試多次獲取回應
        retry_count = 2
        for attempt in range(retry_count):
            try:
                response_text = start_chat_and_get_response(model, message_components)
            except Exception as e:
                print(f"\b ...... \u274C Attempt {attempt + 1} failed for {path}: {e}")
                if attempt < retry_count - 1:
                    time.sleep(5)  # 延遲以進行重試
                else:
                    print(f"\b ...... \u23E9 Skipping {path} after {retry_count} failed attempts.")
                    response_text = None

        if response_text is None:
            continue

        json_response = json.loads(response_text)
        json_response[0]["src_path"] = path
        json_response[0]["allow_move"] = True
        response_list.extend(json_response)

        # 即時存檔
        with open(output_path, "w", encoding="utf-8") as outfile:
            json.dump(response_list, outfile, indent=4, ensure_ascii=False)
            print(f"\b ...... \u2705 saved to {output_path}")
        
        files["uploaded_files"][0].delete()


if __name__ == "__main__":
    summarize_files()
