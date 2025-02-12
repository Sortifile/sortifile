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
import { defineProps, defineEmits } from "vue";

const props = defineProps({ modelValue: Array });
const emit = defineEmits(["update:modelValue"]);

// 檔案樹結構選項
// prettier-ignore
const structureOptions = [
  { key: "year", name: "年份", description: "以年份作為主要分類，如2024/2025" },
  { key: "category", name: "類別", description: "依據用途分類，如作業/專案/報告",},
  { key: "course", name: "課程名稱", description: "按課程分類，如線性代數、物理", },
  { key: "project", name: "專案名稱", description: "如『機器學習專題』或『網站設計』", },
  { key: "date", name: "日期", description: "依日期排序，如2024-02-10" },
  { key: "version", name: "版本號", description: "不同版本的管理，如v1.0、v2.0", },
  { key: "team", name: "小組名稱", description: "如『專題A組』或『報告B組』" },
  { key: "tag", name: "標籤", description: "透過標籤進行分類，如『重要』『草稿』", },
  { key: "file_type", name: "檔案類型", description: "按照檔案性質分類，如PDF、Word", },
  { key: "hello_world", name: "思諾這裡選項我亂生的幫我想正確的", description: "Hello World", },
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
  background: #eef;
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
