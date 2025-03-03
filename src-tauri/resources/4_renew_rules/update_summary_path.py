# 用途：更新 file_summary.json 中的 src_path，以符合 file_movements.json 中的檔案移動紀錄

import json

# 讀取 JSON 資料
with open("4_renew_rules/file_movements.json", "r", encoding="utf-8") as file:
    file_movements = json.load(file)

with open("4_renew_rules/file_summary.json", "r", encoding="utf-8") as file:
    file_summary = json.load(file)

# 建立 src_path 到 new_path 的對應字典
path_mapping = {}
for movement in reversed(file_movements["file_movements"]):
    # 確保新的 new_path 替換舊的 src_path
    current_src = movement["src_path"]
    current_new = movement["new_path"]
    # 如果檔案曾經多次移動，更新到最新的路徑
    while current_new in path_mapping:
        current_new = path_mapping[current_new]
    path_mapping[current_src] = current_new
    

# 更新 file_summary 的 src_path
for summary in file_summary:
    if summary["src_path"] in path_mapping:
        summary["src_path"] = path_mapping[summary["src_path"]]

# 輸出更新後的 file_summary
with open("4_renew_rules/updated_file_summary.json", "w", encoding="utf-8") as file:
    json.dump(file_summary, file, ensure_ascii=False, indent=2)

print("更新完成，結果已儲存至 updated_file_summary.json")
