<template>
  <div>
    <h1>Global Settings</h1>

    <!-- API KEYS Section -->
    <div class="setting-item">
      <label for="api-keys">API KEYS:</label>
      <el-input
        id="api-keys"
        v-model="apiKey"
        style="width: 200px"
        placeholder="Enter your API key"
      />
    </div>

    <!-- Dark Mode Section (改成三選一) -->
    <!-- <div class="setting-item">
      <label for="theme-mode">Theme Mode:</label>
      <el-radio-group v-model="themeMode">
        <el-radio label="system">System</el-radio>
        <el-radio label="light">Light</el-radio>
        <el-radio label="dark">Dark</el-radio>
      </el-radio-group>
    </div> -->

    <!-- Buttons -->
    <div class="button-group">
      <el-button type="primary" @click="saveSettings">Save</el-button>
      <el-button type="danger" @click="resetSettings">Reset</el-button>
    </div>
  </div>
</template>

<script setup>
import { invoke } from "@tauri-apps/api/core";
import { ElMessage } from "element-plus";
import { ref, onMounted } from "vue";

const apiKey = ref("");
// const themeMode = ref("system");

// 在元件載入時載入設定
onMounted(async () => {
  invoke("get_api_key")
    .then((key) => {
      apiKey.value = key;
    })
    .catch((error) => {
      console.error(error);
      ElMessage.error("Failed to get API key");
    });
  // themeMode.value = localStorage.getItem("themeMode") || "system";
  // applyTheme(themeMode.value);
});

// 儲存設定
async function saveSettings() {
  invoke("set_api_key", { apiKey: apiKey.value })
    .then(() => {
      ElMessage.success("API key saved successfully");
    })
    .catch((error) => {
      console.error(error);
      ElMessage.error("Failed to save API key");
    });
  // localStorage.setItem("themeMode", themeMode.value);
  // applyTheme(themeMode.value);
}

// 重置設定
async function resetSettings() {
  invoke("get_api_key")
    .then((key) => {
      apiKey.value = key;
    })
    .catch((error) => {
      console.error(error);
      ElMessage.error("Failed to get API key");
    });
  saveSettings();
}

// 根據選擇的主題模式應用樣式
// function applyTheme(mode) {
//   document.documentElement.classList.remove("dark");
//   if (mode === "dark") {
//     document.documentElement.classList.add("dark");
//   } else if (mode === "system") {
//     const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
//     if (prefersDark) {
//       document.documentElement.classList.add("dark");
//     }
//   }
// }
</script>

<style scoped>
.setting-item {
  margin-bottom: 20px;
  display: flex;
  align-items: center;
}

.setting-item label {
  margin-right: 10px;
  font-weight: bold;
}

.button-group {
  margin-top: 20px;
  display: flex;
  gap: 10px;
}
</style>
