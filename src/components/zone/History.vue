<template>
  <el-dialog
    :model-value="modelValue"
    title="History File Movements"
    width="95%"
    @update:modelValue="updateVisible"
  >
    <div
      v-for="move in historyMoves"
      :key="move.src_path"
      class="card-container"
    >
      <el-card shadow="hover">
        <FileMoveItem :movement="move" />
      </el-card>
    </div>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="updateVisible(false)">Close</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup>
import { defineProps, defineEmits, ref, onMounted } from "vue";
import FileMoveItem from "./FileMoveItem.vue";
import { invoke } from "@tauri-apps/api/core";

const props = defineProps({
  modelValue: { type: Boolean, default: false },
});

const emit = defineEmits(["update:modelValue"]);

const historyMoves = ref([]);

const updateVisible = (value) => {
  emit("update:modelValue", value);
};

onMounted(() => {
  // fetch history moves from backend
  try {
    const result = invoke("get_move_history", { num: 30 });
    historyMoves.value = JSON.parse(result);
  } catch (error) {
    historyMoves.value = [
      {
        src_path: "報告_改.pdf",
        new_path: "113-1/國文/報告/",
        moved_by: "system",
        reason: "blablabla",
      },
      {
        src_path: "報告_改.pdf",
        new_path: "113-1/國文/報告/",
        moved_by: "system",
        reason: "blablabla",
      },
    ];
  }
});
</script>

<style scoped>
.card-container {
  margin: 10px 30px;
  display: flex;
  justify-content: center;
  flex-direction: column;
  overflow-y: auto;
}

el-card {
  margin: 10px 0;
  padding: 0;
  align-self: center;
}
</style>
