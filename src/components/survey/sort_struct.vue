<!-- Problem 3 -->
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
        <h4 class="question-title">3. 你希望大概如何來建立你的檔案樹？</h4>
      </div>
    </template>
    <div class="answer-area">思諾這裡我要留著跟你討論一下要怎麼做</div>
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
