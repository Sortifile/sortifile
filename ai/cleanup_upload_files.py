import os
from dotenv import load_dotenv
import google.generativeai as genai

load_dotenv()
genai.configure(api_key=os.environ["GEMINI_API_KEY"])
print("My files:")
for f in genai.list_files():
    print("Removing:  ", f.name)
    f.delete()
