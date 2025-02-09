<!-- Problem 7 -->

<script setup>
import { defineProps, defineEmits } from "vue";

const props = defineProps({
  modelValue: String, // 單選
});

const emit = defineEmits(["update:modelValue"]);

const options = [
  {
    label: "A. 我重視經常使用的檔案，並傾向於放在容易訪問的位置。",
    value: "A",
  },
  {
    label: "B. 我重視長期存檔與保存，對文件的檔案結構更關心。",
    value: "B",
  },
];

const updateSelection = (value) => {
  emit("update:modelValue", value);
};
</script>

<template>
  <el-card class="question-card" shadow="always">
    <template #header>
      <div class="card-header">
        <h4 class="question-title">7. 下列何者較符合你的想法？</h4>
      </div>
    </template>
    <div class="answer-area">
      <el-radio-group
        v-model="props.modelValue"
        class="radio-group"
        @update:modelValue="updateSelection"
      >
        <el-radio
          v-for="(opt, index) in options"
          :key="index"
          :label="opt.value"
          class="radio-item"
        >
          {{ opt.label }}
        </el-radio>
      </el-radio-group>
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

.radio-group {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}
</style>
