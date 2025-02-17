<script setup>
const props = defineProps({
  modelValue: String, // 單選
});

const emit = defineEmits(["update:modelValue"]);

const options = [
  {
    label:
      "A. 按檔案類型或用途進行分類，例如「報告」、「調查」、「客戶資料」等。",
    value: "A",
  },
  {
    label: "B. 根據當下需求分類，例如「需要優先完成」、「稍後查看」等。",
    value: "B",
  },
  {
    label: "C. 混合或無特定形式",
    value: "C",
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
        <h4 class="question-title">
          2. 當你需要整理工作檔案時，會傾向以下哪種分類？
        </h4>
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
