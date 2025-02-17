<script setup>
const props = defineProps({
  modelValue: Array, // 使用 Array 來存儲多選結果
});

const emit = defineEmits(["update:modelValue"]);

const options = [
  "圖片",
  "影片",
  "音訊",
  "作業",
  "簡報",
  "報告",
  "程式碼",
  "TODO: 思諾再幫我加上其他的或調整選項",
];

const updateSelection = (value) => {
  emit("update:modelValue", value);
};
</script>

<template>
  <el-card class="question-card" shadow="always">
    <template #header>
      <div class="card-header">
        <h4 class="question-title">1. 你的檔案中包含哪些類型的內容？</h4>
      </div>
    </template>
    <div class="answer-area">
      <el-checkbox-group
        :model-value="props.modelValue"
        @update:modelValue="updateSelection"
      >
        <el-checkbox v-for="option in options" :key="option" :label="option">
          {{ option }}
        </el-checkbox>
      </el-checkbox-group>
    </div>
  </el-card>
</template>

<style scoped>
.question-title {
  margin: 0;
  font-size: 16px;
  font-weight: bold;
  color: #333;
}

.question-card {
  margin-top: 16px;
  margin-bottom: 16px;
}

.answer-area {
  margin-top: 8px;
  margin-bottom: 8px;
}
</style>
