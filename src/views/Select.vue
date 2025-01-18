<!-- Select.vue -->
<!-- This is the temporary template file for adding a new page. -->

<template>
  <div class="select-container">
    <!-- 左側滑桿欄 -->
    <div class="sidebar">
      <h2>Browse Files</h2>
      <el-scrollbar height="calc(100vh - 100px)" class="file-scrollbar">
        <p v-for="file in files" :key="file" class="scrollbar-demo-item">{{ file }}</p>
      </el-scrollbar>
    </div>

    <!-- 內容顯示區域 -->
    <div class="content-display">
      <h2>File Details</h2>
      <el-skeleton :rows="5" animated />
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';

// 模擬的檔案清單
const files = ref(['File 1', 'File 2', 'File 3', 'File 4']);
</script>

<style scoped>
.select-container {
  display: flex;
  gap: 20px;
  overflow: hidden;
}

.sidebar {
  width: 200px;
  border-right: 1px solid #dcdcdc;
  padding: 10px;
  max-height: 300px;
  overflow: hidden;
  position: sticky;
  top: 0;
}

.file-scrollbar {
  max-height: calc(100vh - 100px);
  overflow-y: auto;
}

.scrollbar-demo-item {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 50px;
  margin: 10px;
  text-align: center;
  border-radius: 4px;
  background: var(--el-color-primary-light-9);
  color: var(--el-color-primary);
}

.content-display {
  flex: 1;
  padding: 10px;
  overflow: hidden;
}
</style>