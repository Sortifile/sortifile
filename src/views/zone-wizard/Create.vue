<!-- zone-wizard/Create.vue -->

<template>
  <div class="container">
    <h1>Create New Zone</h1>
    <el-form ref="formRef" :model="zoneStore" :rules="rules" label-width="auto">
      <!-- Zone 名稱 -->
      <el-form-item label="Zone Name" prop="zoneName" label-position="left">
        <el-col :span="24">
          <el-input
            v-model="zoneStore.zoneName"
            placeholder="請輸入 Zone 名稱"
          />
        </el-col>
      </el-form-item>

      <!-- 路徑 -->
      <el-form-item
        label="Root Folder Path"
        prop="rootPath"
        label-position="left"
      >
        <el-col :span="19">
          <el-input
            v-model="zoneStore.rootPath"
            placeholder="請選擇路徑"
            readonly
          />
        </el-col>
        <el-col :span="5" align="right">
          <el-button type="primary" @click="selectPath" plain>
            選擇路徑
          </el-button>
        </el-col>
      </el-form-item>
      <el-divider />

      <!-- 送出與重設按鈕 -->
      <el-form-item>
        <el-col :span="5">
          <el-button @click="navigateTo('')">上一步</el-button>
        </el-col>
        <el-col :span="19" align="right">
          <el-button @click="resetForm">重設</el-button>
          <el-button type="primary" @click="submitForm">
            建立
            <el-icon class="el-icon--right"><ArrowRight /></el-icon>
          </el-button>
        </el-col>
      </el-form-item>
    </el-form>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import {
  ElForm,
  ElFormItem,
  ElInput,
  ElButton,
  ElCol,
  ElDivider,
  ElMessage,
} from "element-plus";
import { useRouter } from "vue-router";
import { useZoneStore } from "../../store/zone";
import { ArrowRight } from "@element-plus/icons-vue";

const router = useRouter();
const formRef = ref(null);

function navigateTo(page) {
  router.push(`/${page}`);
}

const zoneStore = useZoneStore();

// 表單驗證規則
const rules = {
  // NOTE: 這裡的 required 設為 false，only for development
  // 請將 required 設為 true
  zoneName: [{ required: true, message: "請輸入 Zone 名稱", trigger: "blur" }],
  rootPath: [{ required: true, message: "請選擇路徑", trigger: "blur" }],
};

// 送出表單
const submitForm = () => {
  formRef.value.validate((valid) => {
    if (valid) {
      console.log("Zonestore：", zoneStore.zoneName, zoneStore.rootPath);
      navigateTo("survey");
    }
  });
  navigateTo("survey");
};

// 重設表單
const resetForm = () => {
  zoneStore.resetZone();
  formRef.value.resetFields();
};

// 呼叫 Tauri Plugin Dialog 選擇路徑
const selectPath = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
    });
    if (typeof selected === "string") {
      zoneStore.setPath(selected);
    } else if (Array.isArray(selected) && selected.length > 0) {
      zoneStore.setPath(selected[0]);
    }
  } catch (error) {
    ElMessage.error("選擇路徑時發生錯誤");
    console.error("選擇路徑時發生錯誤：", error);
  }
};
</script>

<style scoped>
.container {
  max-width: 800px;
  margin: 0 auto;
  height: 100%;
  display: flex;
  flex-direction: column;
}
</style>
