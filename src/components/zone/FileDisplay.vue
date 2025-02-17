<template>
  <div>
    <h3>{{ name }}</h3>

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
    <pre>{{ summary }}</pre>
  </div>
</template>

<script setup>
const props = defineProps({
  name: { type: String, default: "" },
  path: { type: String, default: "" },
  summary: { type: JSON, default: {} },
  ignoreSwitch: { type: Boolean, default: false },
  isInheritedIgnore: { type: Boolean, default: false },
});

const emits = defineEmits(["toggle-ignore"]);

function onSwitchChange(val) {
  emits("toggle-ignore", { path: props.path, shouldIgnore: val });
}
</script>

<style scoped>
/* 依需要調整樣式 */
</style>
