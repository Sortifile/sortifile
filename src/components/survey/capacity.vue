<script setup>
const props = defineProps({
  modelValue: Number, // 滑桿綁定數值
});

const emit = defineEmits(["update:modelValue"]);

const updateSlider = (value) => {
  emit("update:modelValue", value);
};
</script>

<template>
  <el-card class="question-card" shadow="always">
    <template #header>
      <div class="card-header">
        <h4 class="question-title">
          5. 你希望一個資料夾最多裝多少資料之後，需要建立子資料夾來分類？
        </h4>
      </div>
    </template>
    <div class="answer-area">
      <el-slider
        :model-value="props.modelValue"
        @update:modelValue="updateSlider"
        :min="10"
        :max="150"
        show-stops
        show-tooltip
        :step="10"
        class="slider"
      />
      <div class="slider-labels">
        <span>少量 (10 個檔案) </span>
        <span>大量 (150 個檔案) </span>
      </div>
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
  text-align: center;
}

.slider-labels {
  display: flex;
  justify-content: space-between;
  font-size: 14px;
  margin-top: 4px;
}
</style>
