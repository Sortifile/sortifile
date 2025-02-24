from pathlib import Path
import sys

# 將專案根目錄添加到 sys.path
project_root = Path(__file__).resolve().parent.parent
sys.path.append(str(project_root))

from utils import *

def sort_files(
    system_prompt_path="3_sort_files/system_prompt.md",
    user_prompt_path="3_sort_files/user_prompt.txt",
    path_to_sort=".",
    rule_path="3_sort_files/rule.json",
    file_summary_path="3_sort_files/file_summary.json",
    history_file_movements_path="3_sort_files/history_file_movements.json",
    output_path="3_sort_files/respond.json",
    model_name="gemini-exp-1206",
    generation_config=None,
):
    if generation_config is None:
        generation_config = {
            "temperature": 1,
            "top_p": 0.9,
            "top_k": 64,
            "max_output_tokens": 32768,
            "response_mime_type": "application/json",
        }

    setAPIKeyFromEnv()
    with open(user_prompt_path, "r", encoding="utf-8") as file:
        user_prompt = file.read().replace("PATH_TO_SORT", path_to_sort)

    model = configure_generation_model(system_prompt_path, model_name, generation_config)
    rule_json, file_summarize, history_file_movements = upload_files(rule_path, file_summary_path, history_file_movements_path)
    message_components = [
        user_prompt, "\nrule: ", rule_json, "\nfile_summary: ", file_summarize,
        "\nhistory_file_movements: ", history_file_movements
    ]
    response_text = start_chat_and_get_response(model, message_components)
    if response_text == "ERROR":
        return
    save_json(output_path, response_text)

def main():
    # 检查是否传入足够的参数（sys.argv[0] 是脚本名）
    if len(sys.argv) < 8:
        print("Usage: sort_files.py <system_prompt> <user_prompt> <path_to_sort> <rule_path> <file_summary_path> <history_file_movements_path> <output_file>")
        print("    - Note: The GEMINI_API_KEY environment variable must be set.")
        sys.exit(1)

    system_prompt_path = sys.argv[1]
    user_prompt_path = sys.argv[2]
    path_to_sort = sys.argv[3]
    rule_path = sys.argv[4]
    file_summary_path = sys.argv[5]
    history_file_movements_path = sys.argv[6]
    output_file_path = sys.argv[7]

    sort_files(
        system_prompt_path=system_prompt_path,
        user_prompt_path=user_prompt_path,
        path_to_sort=path_to_sort,
        rule_path=rule_path,
        file_summary_path=file_summary_path,
        history_file_movements_path=history_file_movements_path,
        output_path=output_file_path,
    )

if __name__ == "__main__":
    main()
