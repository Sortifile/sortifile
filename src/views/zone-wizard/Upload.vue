<script setup>
import { useRouter } from "vue-router";
import { ElButton, ElUpload } from "element-plus";

const router = useRouter();

function navigateTo(page) {
  router.push(`/${page}`);
}

const uploadFiles = (file) => {
  console.log("Uploaded folder:", file);
};
</script>

<template>
  <div>
    <h1>Upload</h1>

    <!-- Upload 功能 -->
    <el-upload
      action="#"
      :auto-upload="false"
      directory
      :on-change="uploadFiles"
      class="upload-section"
    >
      <el-button type="primary">Click to Upload Folder</el-button>
    </el-upload>

    <!-- 跳轉介面的按鈕 -->
    <el-button type="success" @click="navigateTo('survey')">Next</el-button>
  </div>
</template>

<style scoped>
.upload-section {
  margin-bottom: 20px;
}
</style>
