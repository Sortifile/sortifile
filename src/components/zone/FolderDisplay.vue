<template>
  <div>
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
    <p>這是資料夾，rrrr請選擇一個檔案查看內容：</p>
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

/**
 * 當 Switch 改變時，emit 事件給父層
 * 父層再去改變實際的忽略清單
 */
function onSwitchChange(val) {
  emits("toggle-ignore", { path: props.selectedKey, shouldIgnore: val });
}
</script>
