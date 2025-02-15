<template>
  <div>
    <p>這裡是根目錄的資訊</p>
    <pre>{{ fileInfo }}</pre>
  </div>
</template>

<script setup>
/**
 * 父層傳入的屬性 (fileInfo)
 */
defineProps({
  fileInfo: {
    type: String,
    default: "",
  },
});
</script>

<style scoped>
/* 依需要調整樣式 */
</style>
