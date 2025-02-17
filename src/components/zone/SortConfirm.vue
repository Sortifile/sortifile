<template>
  <el-dialog
    :model-value="modelValue"
    title="Confirm File Movements"
    width="95%"
    @update:modelValue="updateVisible"
    :before-close="handleClose"
  >
    <el-table
      ref="tableRef"
      :data="sortResult.file_movements"
      style="width: 100%"
      @selection-change="handleSelectionChange"
    >
      <el-table-column type="selection" width="35" />
      <el-table-column>
        <template #default="scope">
          <FileMoveItem :movement="scope.row" />
        </template>
      </el-table-column>
    </el-table>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClose">Cancel</el-button>
        <el-button type="primary" @click="handleConfirm">Confirm</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup>
import { defineProps, defineEmits, ref, onMounted, watch } from "vue";
import { ElMessage, ElMessageBox, SCOPE } from "element-plus";
import FileMoveItem from "./FileMoveItem.vue";

const props = defineProps({
  modelValue: { type: Boolean, default: false },
  sortResult: {
    type: Object,
    default: () => ({
      file_movements: [],
    }),
  },
});

const emit = defineEmits(["update:modelValue", "confirm-selection"]);
const selectedMoves = ref([]);
const tableRef = ref(null); // 用來存放 el-table 的引用

// 監聽使用者的勾選變化
const handleSelectionChange = (selection) => {
  console.log("selection", selection);
  selectedMoves.value = selection; // 更新選擇的項目
};

const updateVisible = (value) => {
  emit("update:modelValue", value);
};

// **當對話框顯示時，預設全選**
watch(
  () => props.modelValue,
  (newVal) => {
    if (newVal) {
      setTimeout(() => {
        if (tableRef.value) {
          tableRef.value.toggleAllSelection(); // 預設全選
        }
      }, 100); // 加入小延遲確保資料載入
    }
  },
);

function handleClose() {
  ElMessageBox.confirm("Are you sure to cancel sorting process?")
    .then(() => {
      ElMessage({ message: "Moving Files Canceled.", type: "warning" });
      updateVisible(false);
    })
    .catch(() => {});
}

function handleConfirm() {
  if (selectedMoves.value.length === 0) {
    ElMessage.warning("Please select at least one file to move.");
    return;
  }
  emit("confirm-selection", selectedMoves.value); // 將選中的檔案回傳給父元件
  updateVisible(false);
  ElMessage.success("Selected files confirmed.");
}
</script>
