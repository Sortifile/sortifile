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
        show-password
      />
    </div>

    <!-- Dark Mode Section (改成三選一) -->
    <div class="setting-item">
      <label for="theme-mode">Theme Mode:</label>
      <el-radio-group v-model="themeMode">
        <el-radio label="system">System</el-radio>
        <el-radio label="light">Light</el-radio>
        <el-radio label="dark">Dark</el-radio>
      </el-radio-group>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, onMounted } from "vue";

const apiKey = ref("");

// 預設值 (可從 localStorage 或其它地方載入)
const themeMode = ref("system"); // system | light | dark

// 在元件載入時，若 themeMode = 'system'，可初始化一次
onMounted(() => {
  applyTheme(themeMode.value);
});

// 監聽 themeMode 變化
watch(themeMode, (newMode) => {
  applyTheme(newMode);
  // 若需要儲存使用者選擇，可用 localStorage.setItem('themeMode', newMode)
});

// 根據當前選擇套用主題
function applyTheme(mode) {
  // 先移除任何可能殘留的 dark class
  document.documentElement.classList.remove("dark");

  if (mode === "dark") {
    document.documentElement.classList.add("dark");
  } else if (mode === "system") {
    // 自動偵測系統偏好
    const prefersDark = window.matchMedia(
      "(prefers-color-scheme: dark)",
    ).matches;
    if (prefersDark) {
      document.documentElement.classList.add("dark");
    }
  }
}
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
</style>
