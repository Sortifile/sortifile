<template>
  <div class="file-move-item">
    <div
      class="move-type"
      :style="{ backgroundColor: moveType.color, color: moveType.fontColor }"
    >
      {{ moveType.text }}
    </div>

    <div class="file-info">
      <div class="row">
        <el-tag type="info" effect="plain" class="no-border-tag">
          原路徑：
        </el-tag>
        <pre>{{ movement.src_path }}</pre>
      </div>
      <div class="row">
        <el-tag type="success" effect="plain" class="no-border-tag">
          新路徑：
        </el-tag>
        <pre>{{ movement.new_path }}</pre>
      </div>
      <div class="row">
        <el-tag type="danger" effect="plain" class="no-border-tag">
          移動原因：
        </el-tag>
        <pre>{{ movement.reason }}</pre>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from "vue";
import { ElTag } from "element-plus";

const props = defineProps({
  movement: {
    type: Object,
    required: true,
  },
});

const moveType = computed(() => {
  return props.movement.moved_by === "system"
    ? { text: "AI move", color: "#ffebc5", fontColor: "#d08700" }
    : { text: "User move", color: "#d5f5c7", fontColor: "#2d862d" };
});
</script>

<style scoped>
.file-move-item {
  width: 100%;
  margin: 10px 0;
  position: relative;
  padding: 0;
}

.move-type {
  position: absolute;
  top: 8px;
  right: 12px;
  padding: 6px 10px;
  font-size: 12px;
  font-weight: bold;
  border-radius: 6px;
}

.file-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.row {
  display: flex;
  align-items: center;
  gap: 6px;
}

pre {
  margin: 0;
  padding: 4px 8px;
  font-size: smaller;
  font-family: monospace;
  background: #f5f5f5;
  border-radius: 4px;
  display: block;
  max-width: 100%;
  overflow-x: auto;
  white-space: nowrap;
}

.no-border-tag {
  border: none !important;
  background-color: transparent !important;
}
</style>
