import os
import json
from dotenv import load_dotenv
import google.generativeai as genai

def load_environment():
    load_dotenv()
    genai.configure(api_key=os.environ["GEMINI_API_KEY"])

def configure_generation_model(system_prompt_path, model_name, generation_config):
    with open(system_prompt_path, "r", encoding="utf-8") as file:
        content = file.read()
    return genai.GenerativeModel(
        model_name=model_name,
        generation_config=generation_config,
        system_instruction=content,
    )

def upload_files(*file_paths, mime_type="text/plain"):
    return [genai.upload_file(path=file_path, mime_type=mime_type) for file_path in file_paths]

def start_chat_and_get_response(model, message_components):
    chat_session = model.start_chat(history=[])
    response = chat_session.send_message(message_components)
    return response.text

def save_response(output_path, response_text):
    print(f"回應：\n{response_text}")
    try:
        parsed_json = json.loads(response_text)
        with open(output_path, "w", encoding="utf-8") as outfile:
            json.dump(parsed_json, outfile, indent=4, ensure_ascii=False)
        print(f"格式化的 JSON 已寫入 {output_path}")
    except json.JSONDecodeError:
        with open(output_path, "w", encoding="utf-8") as outfile:
            outfile.write(response_text)
        print(f"回應無法解析為 JSON，已原樣寫入 {output_path}")
