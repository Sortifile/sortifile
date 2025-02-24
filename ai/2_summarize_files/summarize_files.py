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

# def process_files_in_directory(directory):
#     """
#     遞迴遍歷目錄並根據檔案類型執行不同的處理。

#     Args:
#         directory (str): 要遍歷的目錄路徑。

#     Returns:
#         list: 包含檔案相對路徑及其副檔名的列表。
#     """
#     base_path = Path(directory)
#     file_list = []
#     for file_path in base_path.rglob("*"):  # 遞迴遍歷
#         if file_path.is_file() and not file_path.name.startswith("."):  # 確保是檔案
#             relative_path = file_path.relative_to(base_path)  # 取得相對路徑
#             file_extension = file_path.suffix.lower()  # 取得副檔名（小寫）
            
#             file_list.append({"path": str(relative_path), "extension": file_extension})

#     return file_list

# def read_file_list(file_list_path):
#     """
#     讀取包含檔案路徑的 txt 檔案，並轉換為陣列。

#     Args:
#         file_list_path (str): txt 檔案的路徑。

#     Returns:
#         list: 包含檔案路徑的列表。
#     """
#     with open(file_list_path, "r", encoding="utf-8") as file:
#         file_list = [line.strip() for line in file if line.strip()]
#     file_list = [{"path": file_path, "extension": Path(file_path).suffix.lower()} for file_path in file_list]
#     return file_list

def summarize_files(
    system_prompt_path="2_summarize_files/system_prompt.md",
    user_prompt_path="2_summarize_files/user_prompt.txt",
    root_path="2_summarize_files/sample",
    rule_path="2_summarize_files/rule.json",
    file_path="",
    output_path="2_summarize_files/respond.json",
    model_name="gemini-exp-1206",
    generation_config=None,
):
    """
    對目錄中的檔案進行摘要處理。

    Args:
        system_prompt_path (str): 系統提示檔案的路徑。
        user_prompt_path (str): 用戶提示檔案的路徑。
        root_path (str): 要處理的目錄路徑。
        file_list_path (str): 包含要處理的檔案相對路徑的 txt 檔案。
        output_path (str): 輸出結果的檔案路徑。
        model_name (str): 使用的生成模型名稱。
        generation_config (dict, optional): 模型生成的配置選項。

    Returns:
        None
    """
    generation_config = generation_config or DEFAULT_GENERATION_CONFIG

    setAPIKeyFromEnv()

    with open(user_prompt_path, "r", encoding="utf-8") as file:
        user_prompt = file.read()

    model = configure_generation_model(system_prompt_path, model_name, generation_config)
    file_list = [{"path": file_path, "extension": Path(file_path).suffix.lower()}]
    # response_list = []

    for files in file_list:
        path = files["path"]
        extension = files["extension"]
        print(f"Processing {path} with extension {extension}")
        files["uploaded_files"] = upload_files(os.path.join(root_path, path), mime_type=mimetypes.types_map.get(extension, "application/octet-stream"))
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
        # json_response["src_path"] = path
        # json_response["allow_move"] = True
        # response_list.extend(json_response)

        # 即時存檔
        with open(output_path, "w", encoding="utf-8") as outfile:
            json.dump(json_response, outfile, indent=4, ensure_ascii=False)
            print(f"\b ...... \u2705 saved to {output_path}")
        
        files["uploaded_files"][0].delete()

def main():
    # 檢查是否傳入足夠的參數（sys.argv[0] 是腳本名）
    if len(sys.argv) < 6:
        print("Usage: summarize_files.py <system_prompt> <user_prompt> <root_path> <file_path> <output_file>")
        print("    - Note: The GEMINI_API_KEY environment variable must be set.")
        sys.exit(1)

    system_prompt_path = sys.argv[1]
    user_prompt_path = sys.argv[2]
    root_path = sys.argv[3]
    file_path = sys.argv[4]
    output_path = sys.argv[5]
    summarize_files(
        system_prompt_path=system_prompt_path,
        user_prompt_path=user_prompt_path,
        root_path=root_path,
        file_path=file_path,
        output_path=output_path,
    )

if __name__ == "__main__":
    main()
