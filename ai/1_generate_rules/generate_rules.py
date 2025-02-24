from pathlib import Path
import sys

# 將專案根目錄添加到 sys.path
project_root = Path(__file__).resolve().parent.parent
sys.path.append(str(project_root))

from utils import *

def generate_rules(
    system_prompt_path="1_generate_rules/system_prompt.md",
    form_respond_path="1_generate_rules/form_respond.json",
    form_question_path="1_generate_rules/form_question.json",
    output_path="1_generate_rules/respond.json",
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
    model = configure_generation_model(system_prompt_path, model_name, generation_config)
    uploaded_files = upload_files(form_respond_path, form_question_path)
    message_components = [
        "form_question: ", uploaded_files[1], "\nform_respond: ", uploaded_files[0]
    ]
    response_text = start_chat_and_get_response(model, message_components)
    if response_text == "ERROR":
        return
    save_json(output_path, response_text)

def main():
    # 检查是否传入足够的参数（sys.argv[0] 是脚本名）
    if len(sys.argv) < 5:
        print("Usage: process_json.py <system_prompt> <question_file> <response_file> <output_file>")
        print("    - Note: The GEMINI_API_KEY environment variable must be set.")
        sys.exit(1)

    system_prompt_path = sys.argv[1]
    question_file_path = sys.argv[2]
    response_file_path = sys.argv[3]
    output_file_path = sys.argv[4]
    generate_rules(
        system_prompt_path=system_prompt_path,
        form_respond_path=response_file_path,
        form_question_path=question_file_path,
        output_path=output_file_path,
    )

if __name__ == "__main__":
    main()
