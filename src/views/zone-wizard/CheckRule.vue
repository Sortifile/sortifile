<script setup>
import { useRouter } from "vue-router";
import { ElButton, ElUpload } from "element-plus";
import { ArrowRight } from "@element-plus/icons-vue";

const router = useRouter();

function navigateTo(page) {
  router.push(`/${page}`);
}
</script>

<template>
  <div>
    <h1>Check</h1>
    <!-- 跳轉介面的按鈕 -->
    <el-button type="primary" @click="navigateTo('select')"
      >Next<el-icon class="el-icon--right"><ArrowRight /></el-icon
    ></el-button>
  </div>
</template>

<script setup></script>

<style scoped>
.upload-section {
  margin-bottom: 20px;
}
</style>
