<template>
  <div class="container">
    <el-row justify="space-between" align="middle" :gutter="35">
      <el-col :span="21">
        <h1>Survey</h1>
      </el-col>
      <el-col :span="3">
        <el-button plain @click="resetForm"> Reset </el-button>
      </el-col>
    </el-row>
    <el-text class="mx-1" type="info">
      Answer the questions based on how you want to structure this zone
    </el-text>

    <file_types v-model="formResponse.file_types" />
    <logic_habit v-model="formResponse.logic_habit" />
    <sort_struct v-model="formResponse.sort_struct" />
    <folder_depth v-model="formResponse.folder_depth" />
    <capacity v-model="formResponse.capacity" />
    <naming_style
      v-model:naming="formResponse.naming"
      v-model:dateFormat="formResponse.date_format"
      v-model:fileNameRule="formResponse.filename_letter_rule"
    />
    <archival_tendency v-model="formResponse.archival_tendency" />

    <!-- 跳轉介面的按鈕 -->
    <div style="margin-top: 20px">
      <el-row :gutter="20" justify="space-between">
        <el-col :span="3">
          <el-button type="default" @click="navigateTo('create')">
            <el-icon class="el-icon--left"><ArrowLeft /></el-icon>
            Back
          </el-button>
        </el-col>
        <el-col :span="3">
          <el-button
            type="primary"
            v-loading.fullscreen.lock="loading"
            @click="submitForm"
          >
            Submit
            <el-icon class="el-icon--right"><ArrowRight /></el-icon>
          </el-button>
        </el-col>
      </el-row>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import { ElButton, ElMessage, ElLoading } from "element-plus";
import { ArrowRight, ArrowLeft } from "@element-plus/icons-vue";
import { invoke } from "@tauri-apps/api/core";
import { useFormStore } from "../../store/form";
import { useRuleStore } from "../../store/rule";
import { useZoneStore } from "../../store/zone";

import file_types from "../../components/survey/file_types.vue";
import logic_habit from "../../components/survey/logic_habit.vue";
import sort_struct from "../../components/survey/sort_struct.vue";
import folder_depth from "../../components/survey/folder_depth.vue";
import capacity from "../../components/survey/capacity.vue";
import naming_style from "../../components/survey/naming_style.vue";
import archival_tendency from "../../components/survey/archival_tendency.vue";
import { cloneDeep } from "lodash";

const router = useRouter();
const formStore = useFormStore();
const ruleStore = useRuleStore();
const zoneStore = useZoneStore();
const loading = ref(false);

const initialFormState = {
  file_types: [],
  logic_habit: "",
  sort_struct: [],
  folder_depth: 5,
  capacity: 30,
  naming: [],
  date_format: "YYYYMMDD",
  filename_letter_rule: "",
  archival_tendency: "",
};

const formResponse = ref(cloneDeep(initialFormState));

const resetForm = () => {
  formResponse.value = cloneDeep(initialFormState);
  console.log("表單已重置:", formResponse.value);
};

const submitForm = async () => {
  loading.value = true;
  try {
    // 1. 存入 Pinia 的 surveyData
    formStore.setFormResponse(formResponse.value);
    ruleStore.resetRule();

    // 2. 呼叫 Tauri API 生成 rule.json
    invoke("ai_create_rule", {
      zoneName: zoneStore.zoneName,
      zonePath: zoneStore.rootPath,
      createFromStructure: false,
      formResponse: JSON.stringify(formResponse.value),
    })
      .then((ruleJson) => {
        // 存入 Pinia 的 ruleData
        ruleStore.setRule(JSON.parse(ruleJson));
        console.log("AI 生成的規則：", ruleStore.rule);

        // 顯示成功訊息並跳轉
        ElMessage.success("AI 生成規則成功！");
        navigateTo("zone-wizard/CheckRule");
        loading.value = false;
      })
      .error((error) => {
        console.error("API 調用失敗:", error);
        ElMessage.error("生成規則時發生錯誤");
        loading.value = false;
      });
  } catch (error) {
    console.error("API 調用失敗:", error);
    ElMessage.error("生成規則時發生錯誤");
    loading.value = false;
  }
};

const navigateTo = (page) => {
  router.push(`/${page}`);
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
</style>
