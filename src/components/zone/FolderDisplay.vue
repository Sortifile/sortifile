<template>
  <div class="container">
    <h3>{{ name }}/</h3>

    <el-row :gutter="20" class="command-bar">
      <el-col :span="12">
        <div>
          <!-- 使用 :model-value + @change，而不是 v-model -->
          <el-switch
            :model-value="ignoreSwitch"
            active-text="已忽略"
            inactive-text="未忽略"
            :disabled="isInheritedIgnore"
            @change="onSwitchChange"
          />
        </div>
      </el-col>
      <el-col
        :span="12"
        style="display: flex; justify-content: flex-end; padding-right: 20px"
      >
        <el-button
          type="primary"
          @click="emits('sort-folder', props.path, props.name)"
        >
          Sort this folder
        </el-button>
      </el-col>
    </el-row>

    <!-- 資料夾專屬畫面 -->

    <p>這是資料夾，若要檢視檔案摘要，請選擇一個檔案。</p>
  </div>
</template>

<script setup>
const props = defineProps({
  name: { type: String, default: "" },
  path: { type: String, default: "" },
  ignoreSwitch: { type: Boolean, default: false },
  isInheritedIgnore: { type: Boolean, default: false },
});

const emits = defineEmits(["toggle-ignore", "sort-folder"]);

/**
 * 當 Switch 改變時，emit 事件給父層
 * 父層再去改變實際的忽略清單
 */
function onSwitchChange(val) {
  emits("toggle-ignore", props.path, val);
}
</script>

<style scoped>
.contenier {
  width: 100%;
  margin: 0;
  position: relative;
  padding: 0;
}
.command-bar {
  max-width: 100%;
  width: 100%;
  margin: 15px, 0;
}
</style>
