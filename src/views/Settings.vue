<!-- Settings.vue -->
<!-- This is the temporary template file for adding a new page. -->

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

    <!-- Dark Mode Section -->
    <div class="setting-item">
      <label for="dark-mode">Dark Mode:</label>
      <el-switch id="dark-mode" v-model="isDarkMode" />
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";

// Define reactive properties for the settings
const apiKey = ref("");
const isDarkMode = ref(false);
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
