<template>
  <div class="container">
    <el-row justify="space-between" align="middle" :gutter="20">
      <el-col :span="18">
        <h1>Confirming Zone Rules</h1>
      </el-col>
      <el-col :span="2">
        <el-button type="warning" @click="handleCancel"> Cancel </el-button>
      </el-col>
      <el-col :span="2">
        <el-button plain @click="handleReset"> Reset </el-button>
      </el-col>
    </el-row>
    <el-text class="mx-1" type="info">
      Check if all the rules match your preference
    </el-text>

    <!-- 分類指標 -->
    <h3>分類指標</h3>
    <el-form label-position="top">
      <el-row :gutter="20">
        <el-col :span="8">
          <el-form-item label="Sorting Entropy">
            <el-slider
              v-model="ruleData.index.sorting_entropy"
              :min="0"
              :max="10"
            />
          </el-form-item>
        </el-col>
        <el-col :span="8">
          <el-form-item label="Naming Complexity">
            <el-slider
              v-model="ruleData.index.naming_complexity"
              :min="0"
              :max="10"
            />
          </el-form-item>
        </el-col>
        <el-col :span="8">
          <el-form-item label="Archival Tendency">
            <el-slider
              v-model="ruleData.index.archival_tendency"
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
          v-for="(rule, idx) in ruleData.natural_language_rules"
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
          <el-button type="default" @click="handleCancel"> Cancel </el-button>
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
import { ElMessage, ElMessageBox } from "element-plus";
import { ArrowRight, ArrowLeft } from "@element-plus/icons-vue";
import { useRouter } from "vue-router";
import { useZoneStore } from "../../store/zone";
import { useRuleStore } from "../../store/rule";
import { useFormStore } from "../../store/form";
import { storeToRefs } from "pinia";

const router = useRouter();
const zoneStore = useZoneStore();
const ruleStore = useRuleStore();
const formStore = useFormStore();

function navigateTo(page) {
  router.push(`/${page}`);
}

// 表單數據
const { zoneName, rootPath } = storeToRefs(zoneStore);
const { formResponse, formQuestion } = storeToRefs(formStore);
import { cloneDeep, invoke } from "lodash";
import { ru, tr } from "element-plus/es/locales.mjs";
const ruleData = ref(cloneDeep(ruleStore.rule));

// 全選和部分選邏輯
const selectedRules = ref([...ruleData.value.natural_language_rules]);
const checkAll = ref(true);
const isIndeterminate = ref(false);
const isWarn = ref(false);
const MIN_REQUIRED = 4;

// 全選 / 全不選
const handleCheckAllChange = (val) => {
  selectedRules.value = val ? [...ruleData.value.natural_language_rules] : [];
  isWarn.value = selectedRules.value.length < MIN_REQUIRED;
  isIndeterminate.value = false;
};

// 單獨選擇邏輯
const handleRulesChange = (value) => {
  const total = ruleData.value.natural_language_rules.length;
  const checkedCount = value.length;
  isWarn.value = checkedCount < MIN_REQUIRED;
  checkAll.value = checkedCount === total;
  isIndeterminate.value = checkedCount > 0 && checkedCount < total;
};

// 取消
const handleCancel = () => {
  ElMessageBox.confirm("Are you sure you want to cancel?", "Warning", {
    confirmButtonText: "Yes",
    cancelButtonText: "No",
    type: "warning",
  }).then(() => {
    navigateTo("zone");
  });
};

const handleReset = () => {
  ruleData.value = cloneDeep(ruleStore.rule);
  selectedRules.value = [...ruleData.value.natural_language_rules];
  checkAll.value = true;
  isWarn.value = false;
  isIndeterminate.value = false;
};

// 提交邏輯
const handleSubmit = () => {
  if (selectedRules.value.length < MIN_REQUIRED) {
    ElMessage.error(`至少需要選擇 ${MIN_REQUIRED} 項規則！`);
    return;
  }
  console.log("Index 部分:", ruleData.value.index);
  console.log("選擇的規則:", selectedRules.value);
  ruleData.value.natural_language_rules = selectedRules.value;

  // 1. 存入 Pinia 的 ruleData
  ruleStore.setRule(ruleData.value);

  try {
    invoke("set_zone_rules", {
      zonePath: rootPath.value,
      rules: ruleData.value,
    });
    ElMessage.success("Zone 規則已更新！");
  } catch (error) {
    console.error("API 調用失敗:", error);
    ElMessage.error("更新規則時發生錯誤");
  }

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
