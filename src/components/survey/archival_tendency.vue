<!-- Problem 7 -->

<script setup>
import { defineProps, defineEmits } from "vue";

const props = defineProps({
  modelValue: [String, Number, Array], // 綁定輸入值
});

const emit = defineEmits(["update:modelValue"]);
</script>

<template>
  <el-card class="question-card" shadow="always">
    <template #header>
      <div class="card-header">
        <h4 class="question-title">7. 下列何者較符合你的想法？</h4>
      </div>
    </template>
    <div class="answer-area">test</div>
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
</style>
