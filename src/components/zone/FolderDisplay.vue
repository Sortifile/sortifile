<template>
  <div>
    <h3>{{ name }}</h3>

    <!-- 資料夾專屬畫面 -->
    <div class="ignore-toggle">
      <!-- 使用 :model-value + @change，而不是 v-model -->
      <el-switch
        :model-value="ignoreSwitch"
        active-text="已忽略"
        inactive-text="未忽略"
        :disabled="isInheritedIgnore"
        @change="onSwitchChange"
      />
    </div>
    <p>這是資料夾，請選擇一個檔案查看內容。</p>
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
  selectedKey: { type: String, default: "" },
});

const emits = defineEmits(["toggle-ignore"]);

/**
 * 當 Switch 改變時，emit 事件給父層
 * 父層再去改變實際的忽略清單
 */
function onSwitchChange(val) {
  emits("toggle-ignore", { path: props.selectedKey, shouldIgnore: val });
}
</script>
