<script setup>
import { useRouter } from "vue-router";
import { ElButton, ElUpload } from "element-plus";

const router = useRouter();

function navigateTo(page) {
  router.push(`/${page}`);
}
</script>

<template>
  <div>
    <h1>Check</h1>
    <!-- 跳轉介面的按鈕 -->
    <el-button type="success" @click="navigateTo('select')">Next</el-button>
  </div>
</template>

<script setup></script>

<style scoped>
.upload-section {
  margin-bottom: 20px;
}
</style>
