<script setup>
import { useRouter } from "vue-router";
import { ElButton, ElUpload } from "element-plus";
import { ArrowRight, UploadFilled } from "@element-plus/icons-vue";

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
      drag
      action="#"
      :auto-upload="false"
      directory
      :on-change="uploadFiles"
      class="upload-section"
    >
      <el-icon class="el-icon--upload"><upload-filled /></el-icon>
      <div class="el-upload__text">
        Drop file here or <em>click to upload</em>
      </div>
    </el-upload>

    <!-- 跳轉介面的按鈕 -->
    <el-button type="primary" @click="navigateTo('survey')"
      >Next<el-icon class="el-icon--right"><ArrowRight /></el-icon
    ></el-button>
  </div>
</template>

<style scoped>
.upload-section {
  margin-bottom: 20px;
}
</style>
