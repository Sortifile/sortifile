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

    load_environment()
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

if __name__ == "__main__":
    sort_files()
