from pathlib import Path
import sys
import json

# 將專案根目錄添加到 sys.path
project_root = Path(__file__).resolve().parent.parent
sys.path.append(str(project_root))

from utils import *

def renew_rules(
    system_prompt_path="4_renew_rules/system_prompt.md",
    file_movements_path="4_renew_rules/file_movements.json",
    file_summary_path ="4_renew_rules/file_summary.json",
    match_movements_path="4_renew_rules/match_movements.json",
    rule_path="4_renew_rules/rule.json",
    output_path="4_renew_rules/respond.json",
    model_name="gemini-exp-1206",
    generation_config=None,
):
    if generation_config is None:
        generation_config = {
            "temperature": 1,
            "top_p": 0.9,
            "top_k": 64,
            "max_output_tokens": 65536,
            "response_mime_type": "application/json",
        }

    load_environment()
    model = configure_generation_model(system_prompt_path, model_name, generation_config)
    with open(file_movements_path, "r", encoding="utf-8") as file:
        file_movements = file.read()
        file_movements = json.loads(file_movements)["file_movements"]

    with open(file_summary_path, "r", encoding="utf-8") as file:
        file_summary = json.loads(file.read())

    # Match each summary with movements by "src_path"
    matched_data = []
    for summary in file_summary:
        src_path = summary["src_path"]
        matching_movement = next((movement for movement in file_movements if movement["new_path"] == src_path), None)
        if (matching_movement is None):
            continue
        matching_movement["summary"] = summary
        # matching_movement["summary"].pop("src_path")
        # matching_movement["summary"].pop("allow_move")
        matched_data.append(matching_movement)
    
    with open(match_movements_path, "w", encoding="utf-8") as outfile:
        json.dump(matched_data, outfile, indent=4, ensure_ascii=False)
    
    uploaded_files = upload_files(rule_path, match_movements_path)
    message_components = [
        "original rule.json: ", uploaded_files[0], "\nfile_movements: ", uploaded_files[1]
    ]
    response_text = start_chat_and_get_response(model, message_components)
    if response_text == "ERROR":
        return
    save_json(output_path, response_text)

if __name__ == "__main__":
    renew_rules(system_prompt_path="4_renew_rules/refined_system_prompt.md")
