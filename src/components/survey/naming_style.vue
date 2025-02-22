<template>
  <el-card class="question-card" shadow="always">
    <template #header>
      <div class="card-header">
        <h4 class="question-title">6. 你習慣如何命名檔案？</h4>
        <p class="instruction-text">點選選項可加入/刪除，拖移可調整順序</p>
      </div>
    </template>

    <!-- 命名順序區域 -->
    <div class="naming-box">
      <draggable
        v-model="userNamingOrder"
        group="naming"
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
                class="naming-btn"
                @click="removeNaming(index)"
              >
                {{ element.name }}
              </el-button>
            </el-tooltip>
          </span>
        </template>
      </draggable>
    </div>

    <!-- 可選擇的命名要素 -->
    <div class="naming-bank">
      <draggable
        v-model="availableNamingOptions"
        group="naming"
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
                class="naming-btn"
                @click="addNaming(element)"
              >
                {{ element.name }}
              </el-button>
            </el-tooltip>
          </span>
        </template>
      </draggable>
    </div>

    <!-- 日期格式選擇 -->
    <div class="date-format-section">
      <h5 class="date-format-title">日期格式:</h5>
      <el-select v-model="dateFormat" placeholder="選擇日期格式">
        <el-option
          v-for="format in dateFormatOptions"
          :key="format.value"
          :label="format.label"
          :value="format.value"
        />
      </el-select>
    </div>

    <!-- 檔名用字規則 -->
    <div class="file-name-rule-section">
      <h5 class="file-name-rule-title">檔案名稱允許的用字:</h5>
      <el-radio-group v-model="fileNameRule">
        <el-radio
          v-for="rule in fileNameRuleOptions"
          :key="rule.value"
          :label="rule.label"
        >
          {{ rule.label }}
        </el-radio>
      </el-radio-group>
    </div>
  </el-card>
</template>

<script setup>
import { ref, watch } from "vue";
import draggable from "vuedraggable";

const props = defineProps(["namingFormat", "dateFormat", "fileNameRule"]);
const emit = defineEmits([
  "update:namingFormat",
  "update:dateFormat",
  "update:fileNameRule",
]);

// 命名要素選項
// prettier-ignore
const namingOptions = [
  { key: "date", name: "日期", description: "標記該版本日期" },
  { key: "timestamp", name: "時間戳", description: "標記時間戳記，如 20250211-153045" },
  { key: "id", name: "編號/代號", description: "簡短編號或代號，如 hw1、lab2"},
  { key: "content", name: "檔案內容", description: "簡述檔案的標題和主要內容，如『期末報告-氣候變遷』、『營隊第三次籌會記錄』" },
  { key: "source", name: "來源", description: "檔案的來源，如檔案的來源，如某科目、某堂課程或特定資料庫" },
  { key: "usage", name: "用途", description: "檔案的用途，如收藏、研究、報告或參考資料" },
  { key: "version", name: "版本編號", description: "不同版本或修訂次序的管理，如 v1.0, v2.1、draft、final" },
  { key: "author_info", name: "作者資訊", description: "文件創建者的相關資訊，如作者姓名、學號或團隊名稱" },
  { key: "language", name: "語言", description: "如『EN』『ZH-TW』" },
];

const userNamingOrder = ref([]);
const availableNamingOptions = ref([...namingOptions]);

const addNaming = (option) => {
  if (!userNamingOrder.value.some((item) => item.key === option.key)) {
    userNamingOrder.value.push(option);
    availableNamingOptions.value = availableNamingOptions.value.filter(
      (w) => w.key !== option.key,
    );
  }
};

const removeNaming = (index) => {
  availableNamingOptions.value.push(userNamingOrder.value[index]);
  userNamingOrder.value.splice(index, 1);
};

watch(
  userNamingOrder,
  (newValue) => {
    emit(
      "update:namingFormat",
      newValue.map((item) => item.name),
    );
  },
  { deep: true },
);
// 日期格式選擇
const dateFormat = ref(props.dateFormat || "YYYYMMDD");

// 可選日期格式
const dateFormatOptions = [
  { value: "YYYYMMDD", label: "YYYYMMDD (20250212)" },
  { value: "YYYY-MM-DD", label: "YYYY-MM-DD (2025-02-12)" },
  { value: "MM-DD-YYYY", label: "MM-DD-YYYY (02-12-2025)" },
  { value: "DD-MM-YYYY", label: "DD-MM-YYYY (12-02-2025)" },
  { value: "DD MMMM YYYY", label: "DD MMMM YYYY (12 February 2025)" },
  { value: "DD MMM YYYY", label: "DD MMM YYYY (12 Feb 2025)" },
];

// 監聽 dateFormat 變化，通知父元件
watch(dateFormat, (newValue) => {
  emit("update:dateFormat", newValue);
});

// 檔名用字規則（單選）
const fileNameRule = ref(props.fileNameRule || "allowAny");

// 可選擇的檔名用字規則
const fileNameRuleOptions = [
  { value: "allowChinese", label: "允許任何字符與中文名稱" },
  { value: "allowSpace", label: "允許空格" },
  {
    value: "alphaNumUnderscoreDash",
    label: "僅允許英文、數字、底線 (_)、破折號 (-)",
  },
];

// 監聽 fileNameRule 變化，通知父元件
watch(fileNameRule, (newValue) => {
  emit("update:fileNameRule", newValue);
});
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

.naming-box {
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

.naming-bank {
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

.naming-btn {
  font-size: 16px;
  padding: 8px 12px;
  cursor: pointer;
}
</style>
