<template>
  <div class="container">
    <el-row justify="space-between" align="middle" :gutter="20">
      <el-col :span="16">
        <h1>Confirming Zone Rules</h1>
      </el-col>
      <el-col :span="4">
        <el-button type="danger" plain @click=""> Regenerate Rules </el-button>
      </el-col>
      <el-col :span="2">
        <el-button plain @click=""> Reset </el-button>
      </el-col>
    </el-row>
    <el-text class="mx-1" type="info">Some description</el-text>

    <!-- 分類指標 -->
    <h3>分類指標</h3>
    <el-form label-position="top">
      <el-row :gutter="20">
        <el-col :span="8">
          <el-form-item label="Sorting Entropy">
            <el-slider
              v-model="formData.index.sorting_entropy"
              :min="0"
              :max="10"
            />
          </el-form-item>
        </el-col>
        <el-col :span="8">
          <el-form-item label="Naming Complexity">
            <el-slider
              v-model="formData.index.naming_complexity"
              :min="0"
              :max="10"
            />
          </el-form-item>
        </el-col>
        <el-col :span="8">
          <el-form-item label="Archival Tendency">
            <el-slider
              v-model="formData.index.archival_tendency"
              :min="0"
              :max="10"
            />
          </el-form-item>
        </el-col>
      </el-row>
    </el-form>

    <!-- 自然語言規則 -->
    <h3>自然語言規則</h3>
    <el-form-item>
      <el-checkbox
        v-model="checkAll"
        :indeterminate="isIndeterminate"
        @change="handleCheckAllChange"
      >
        Select All
      </el-checkbox>
      <el-checkbox-group v-model="selectedRules" @change="handleRulesChange">
        <el-checkbox
          v-for="(rule, idx) in formData.natural_language_rules"
          :key="idx"
          :label="rule"
        >
          {{ rule }}
        </el-checkbox>
      </el-checkbox-group>
      <div v-if="isWarn" class="warning-text">
        至少需要選擇 {{ MIN_REQUIRED }} 項規則！
      </div>
    </el-form-item>

    <!-- 確認按鈕 -->
    <div style="margin-top: 20px">
      <el-row :gutter="20" justify="space-between">
        <el-col :span="3">
          <el-button type="default" @click="navigateTo('survey')">
            <el-icon class="el-icon--left"><ArrowLeft /></el-icon>
            Back
          </el-button>
        </el-col>
        <el-col :span="3">
          <el-button type="primary" @click="handleSubmit">
            Confirm
            <el-icon class="el-icon--right"><ArrowRight /></el-icon>
          </el-button>
        </el-col>
      </el-row>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { ElMessage } from "element-plus";
import { ArrowRight, ArrowLeft } from "@element-plus/icons-vue";
import { useRouter } from "vue-router";

const router = useRouter();

function navigateTo(page) {
  router.push(`/${page}`);
}

// 表單數據
const formData = ref({
  index: {
    sorting_entropy: 8,
    naming_complexity: 6,
    archival_tendency: 10,
  },
  natural_language_rules: [
    "Organize files primarily by semester, then by subject, and finally by their intended use.",
    "Homework, reports, and presentations should be the primary file types considered for organization.",
    "Maintain a maximum folder depth of 5 levels to ensure easy navigation and accessibility.",
    "Limit the number of items in a single folder to 30. Create subfolders when this limit is reached.",
    "Prioritize a logical structure for file classification, adhering to a consistent framework.",
    "File names should clearly indicate the name and version when applicable.",
    "Consistently archive files to ensure long-term preservation and structured storage.",
    "New folders should be created based on the semester to reflect the chronological order of academic activities.",
    "Within each semester folder, create subfolders for each subject to categorize coursework.",
    "Further categorize files within each subject folder by their purpose, such as assignments, projects, or study materials.",
  ],
});

// 全選和部分選邏輯
const selectedRules = ref([...formData.value.natural_language_rules]);
const checkAll = ref(true);
const isIndeterminate = ref(false);
const isWarn = ref(false);
const MIN_REQUIRED = 4;

// 全選 / 全不選
const handleCheckAllChange = (val) => {
  selectedRules.value = val ? [...formData.value.natural_language_rules] : [];
  isWarn.value = selectedRules.value.length < MIN_REQUIRED;
  isIndeterminate.value = false;
};

// 單獨選擇邏輯
const handleRulesChange = (value) => {
  const total = formData.value.natural_language_rules.length;
  const checkedCount = value.length;
  isWarn.value = checkedCount < MIN_REQUIRED;
  checkAll.value = checkedCount === total;
  isIndeterminate.value = checkedCount > 0 && checkedCount < total;
};

// 提交邏輯
const handleSubmit = () => {
  if (selectedRules.value.length < MIN_REQUIRED) {
    ElMessage.error(`至少需要選擇 ${MIN_REQUIRED} 項規則！`);
    return;
  }
  console.log("Index 部分:", formData.value.index);
  console.log("選擇的規則:", selectedRules.value);
  navigateTo("zone");
};
</script>

<style scoped>
.container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
  flex-wrap: wrap;
  text-wrap: normal;
}

.warning-text {
  color: red;
  margin-top: 10px;
}

.el-checkbox .el-checkbox__label {
  white-space: normal;
  word-wrap: break-word;
  word-break: break-word;
}
</style>
