<template>
  <el-card class="question-card" shadow="always">
    <template #header>
      <div class="card-header">
        <h4 class="question-title">3. 你希望大概如何來建立你的檔案樹？</h4>
        <p class="instruction-text">點選選項可加入/刪除，拖移可調整順序</p>
      </div>
    </template>

    <!-- 檔案樹結構區域 -->
    <div class="structure-box">
      <draggable
        v-model="userStructureOrder"
        group="structure"
        class="drop-area"
        item-key="key"
        :component-data="{ tag: 'span' }"
      >
        <template #item="{ element, index }">
          <span>
            <el-tooltip
              effect="dark"
              :content="element.description"
              placement="top"
            >
              <el-button
                type="primary"
                class="structure-btn"
                @click="removeStructure(index)"
              >
                {{ element.name }}
              </el-button>
            </el-tooltip>
          </span>
        </template>
      </draggable>
    </div>

    <!-- 可選擇的結構要素 -->
    <div class="structure-bank">
      <draggable
        v-model="availableStructureOptions"
        group="structure"
        class="drag-area"
        item-key="key"
        :component-data="{ tag: 'span' }"
      >
        <template #item="{ element }">
          <span>
            <el-tooltip
              effect="dark"
              :content="element.description"
              placement="top"
            >
              <el-button
                type="default"
                class="structure-btn"
                @click="addStructure(element)"
              >
                {{ element.name }}
              </el-button>
            </el-tooltip>
          </span>
        </template>
      </draggable>
    </div>
  </el-card>
</template>

<script setup>
import { ref, watch } from "vue";
import draggable from "vuedraggable";

const props = defineProps({ modelValue: Array });
const emit = defineEmits(["update:modelValue"]);

// 檔案樹結構選項
// prettier-ignore
const structureOptions = [
  { key: "time", name: "時間", description: "依時間（學期、年份、日期或時間戳記）分類。ex: 分為 2025/2024/2023" },
  { key: "source", name: "來源", description: "檔案的來源，如某堂課程或特定資料庫。ex: 分爲 微積分/物理科/駕訓班" },
  { key: "usage", name: "用途",  description: "依據用途分類。ex:分為 程式/作業/專案/報告",},
  { key: "topic", name: "檔案主題", description: "依據檔案的主題或內容分類。ex: Sortifile專案/上課程式/個人網頁" },
  { key: "version_order", name: "版本/次序", description: "不同版本的管理。ex: 分爲 v1.0/v2.0" },
  { key: "tags", name: "關鍵字標籤", description: "透過個人化的標籤進行分類。ex: 分為 重要/草稿/私人" },
  { key: "file_format", name: "檔案格式類型", description: "按照檔案格式分類。ex: 分為 PDF/Word/Excel" },
  { key: "index", name: "編號", description: "屬於某項特定編號的檔案。ex: 分為 01/02/03" }
];

const userStructureOrder = ref([]);
const availableStructureOptions = ref([...structureOptions]);

const addStructure = (option) => {
  if (!userStructureOrder.value.some((item) => item.key === option.key)) {
    userStructureOrder.value.push(option);
    availableStructureOptions.value = availableStructureOptions.value.filter(
      (w) => w.key !== option.key,
    );
  }
};

const removeStructure = (index) => {
  availableStructureOptions.value.push(userStructureOrder.value[index]);
  userStructureOrder.value.splice(index, 1);
};

watch(
  userStructureOrder,
  (newValue) => {
    emit(
      "update:structureFormat",
      newValue.map((item) => item.name),
    );
  },
  { deep: true },
);
</script>

<style scoped>
.question-title {
  margin: 0;
  font-size: 16px;
  font-weight: bold;
  color: #333;
}

.instruction-text {
  font-size: 14px;
  color: #666;
  margin-top: 4px;
}

.question-card {
  margin-top: 16px;
  margin-bottom: 16px;
}

.structure-box {
  min-height: 65px;
  border: 2px dashed #ccc;
  padding: 10px;
  margin-bottom: 20px;
  display: flex;
  gap: 10px;
  flex-wrap: nowrap;
  overflow-x: hidden;
  background: #f9f9f9;
  border-radius: 8px;
}

.structure-bank {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  padding: 10px;
  border: 2px dashed #ccc;
  background: #fcfad8;
  border-radius: 8px;
}

.drop-area {
  display: flex;
  flex-wrap: nowrap;
  overflow: auto;
  gap: 10px;
}

.drag-area {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.structure-btn {
  font-size: 16px;
  padding: 8px 12px;
  cursor: pointer;
}
</style>
