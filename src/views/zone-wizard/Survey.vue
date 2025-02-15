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
    <el-text class="mx-1" type="info">Some description</el-text>

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
          <el-button type="primary" @click="submitForm">
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
import { ElButton } from "element-plus";
import { ArrowRight, ArrowLeft } from "@element-plus/icons-vue";

import file_types from "../../components/survey/file_types.vue";
import logic_habit from "../../components/survey/logic_habit.vue";
import sort_struct from "../../components/survey/sort_struct.vue";
import folder_depth from "../../components/survey/folder_depth.vue";
import capacity from "../../components/survey/capacity.vue";
import naming_style from "../../components/survey/naming_style.vue";
import archival_tendency from "../../components/survey/archival_tendency.vue";

const router = useRouter();

function navigateTo(page) {
  router.push(`/${page}`);
}

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

const formResponse = ref({ ...initialFormState });

const resetForm = () => {
  formResponse.value = { ...initialFormState };
  console.log("表單已重置:", formResponse.value);
};

const submitForm = () => {
  console.log("提交的表單資料:", formResponse.value);
  navigateTo("zone-wizard/CheckRule");
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
