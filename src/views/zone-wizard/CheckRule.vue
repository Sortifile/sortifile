<template>
  <div class="container">
    <el-row justify="space-between" align="middle" :gutter="20">
      <el-col :span="16">
        <h1>Confirming Zone Rules</h1>
      </el-col>
      <el-col :span="4">
        <el-button type="warning" @click="handleRegenerate">
          Regenerate Rules
        </el-button>
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
      <div style="display: flex; flex-direction: column; width: 100%">
        <el-row :gutter="20">
          <el-col :span="6">
            <el-checkbox
              v-model="checkAll"
              :indeterminate="isIndeterminate"
              @change="handleCheckAllChange"
            >
              Select All
            </el-checkbox>
          </el-col>
          <el-col :span="18">
            <el-text v-show="isWarn" class="warning-text" type="danger"
              >至少需要選擇 {{ MIN_REQUIRED }} 項規則！</el-text
            >
          </el-col>
        </el-row>

        <el-checkbox-group v-model="selectedRules" @change="handleRulesChange">
          <el-row v-for="(rule, idx) in ruleData.natural_language_rules">
            <el-checkbox :key="idx" :value="rule" :label="rule">
              {{ rule }}
            </el-checkbox>
          </el-row>
        </el-checkbox-group>
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
          <el-button
            type="primary"
            @click="handleSubmit"
            v-loading.fullscreen.lock="loading"
          >
            Confirm
            <el-icon class="el-icon--right"><ArrowRight /></el-icon>
          </el-button>
        </el-col>
      </el-row>
    </div>
  </div>
</template>

<script setup>
import { onMounted, ref, watch } from "vue";
import { ElMessage } from "element-plus";
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
import { cloneDeep } from "lodash";
import { invoke } from "@tauri-apps/api/core";
const { rule } = storeToRefs(ruleStore);

const ruleData = ref({
  index: {
    sorting_entropy: 8,
    naming_complexity: 6,
    archival_tendency: 10,
  },
  spec: {
    file_types: [],
    sort_struct: ["學期", "科目", "用途"],
    folder_depth: 5,
    capacity: 30,
    naming_style: ["name", "version"],
    date_format: "YYYYMMDD",
    filename_letter_rule: "none",
  },
  natural_language_rules: [],
});

// 全選和部分選邏輯
const selectedRules = ref([]);
const checkAll = ref(true);
const isIndeterminate = ref(false);
const isWarn = ref(false);
const MIN_REQUIRED = 4;
const loading = ref(false);

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

const handleRegenerate = () => {
  loading.value = true;
  invoke("ai_create_rule", {
    zone_name: zoneStore.zoneName,
    zone_path: zoneStore.rootPath,
    create_from_structure: false,
    form_response: formResponse.value,
  })
    .then((ruleJson) => {
      ruleStore.setRule(JSON.parse(ruleJson));
      console.log("AI 生成的規則：", ruleStore.rule);

      // 顯示成功訊息並跳轉
      ElMessage.success("AI 生成規則成功！");
      loading.value = false;
    })
    .catch((error) => {
      ElMessage.error("AI 生成規則失敗！");
      console.error("AI 生成規則失敗：", error);
      loading.value = false;
    });
};

// 提交邏輯
const handleSubmit = async () => {
  if (selectedRules.value.length < MIN_REQUIRED) {
    ElMessage.error(`至少需要選擇 ${MIN_REQUIRED} 項規則！`);
    return;
  }
  loading.value = true;
  console.log("Index 部分:", ruleData.value.index);
  console.log("選擇的規則:", selectedRules.value);
  ruleData.value.natural_language_rules = selectedRules.value;

  ruleStore.setRule(ruleData.value);

  try {
    // call API to save and create zone
    let config = {
      name: zoneName.value,
      form_question: formQuestion.value,
      form_response: formResponse.value,
      rules: ruleData.value,
      create_from_structure: false,
    };

    await invoke("create_zone", {
      zoneName: zoneName.value,
      rootPath: rootPath.value,
      config: JSON.stringify(config),
      ignore: "",
      rules: JSON.stringify(ruleData.value),
    })
      .then((res) => {
        console.log("API response:", res);
        ElMessage.success("Created rules successfully!");
        navigateTo("zone");
        loading.value = false;
      })
      .catch((error) => {
        console.error("API error:", error);
        ElMessage.error("Error when submitting");
        loading.value = false;
      });
  } catch (error) {
    ElMessage.error("Error when submitting zone rules", error);
    loading.value = false;
  }
};

// 確保 `ruleStore.rule` 載入後更新 `ruleData`
watch(
  rule,
  (newRule) => {
    if (newRule && Object.keys(newRule).length > 0) {
      ruleData.value = cloneDeep(newRule);
      selectedRules.value = [...ruleData.value.natural_language_rules];
      console.log("Updated ruleData:", ruleData.value);
    }
  },
  { immediate: true },
);

// `handleReset` 使用正確的 `rule`
const handleReset = () => {
  ruleData.value = cloneDeep(rule.value);
  selectedRules.value = [...ruleData.value.natural_language_rules];
  checkAll.value = true;
  isIndeterminate.value = false;
  isWarn.value = false;
};

// `onMounted` 初始化
onMounted(() => {
  if (!ruleStore.rule) {
    console.warn("ruleStore.rule is undefined, fallback to default.");
    ruleStore.resetRule();
  }

  ruleData.value = cloneDeep(rule.value);

  if (ruleData.value.natural_language_rules) {
    selectedRules.value = [...ruleData.value.natural_language_rules];
  } else {
    selectedRules.value = []; // 避免 undefined 錯誤
  }

  console.log("Rule Data on mounted:", ruleData.value);
});
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
