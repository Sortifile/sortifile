<template>
  <div>
    <!-- 檔案專屬畫面 -->
    <div class="ignore-toggle">
      <!-- 一樣使用 :model-value + @change -->
      <el-switch
        :model-value="ignoreSwitch"
        active-text="已忽略"
        inactive-text="未忽略"
        :disabled="isInheritedIgnore"
        @change="onSwitchChange"
      />
    </div>
    <p>這是檔案內容：</p>
    <pre>{{ fileInfo }}</pre>
  </div>
</template>

<script setup>
const props = defineProps({
  fileInfo: { type: String, default: "" },
  ignoreSwitch: { type: Boolean, default: false },
  isInheritedIgnore: { type: Boolean, default: false },
  selectedKey: { type: String, default: "" },
});

const emits = defineEmits(["toggle-ignore"]);

function onSwitchChange(val) {
  emits("toggle-ignore", { path: props.selectedKey, shouldIgnore: val });
}
</script>

<style scoped>
/* 依需要調整樣式 */
</style>
