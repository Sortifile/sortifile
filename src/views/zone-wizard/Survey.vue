<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import { ElButton, ElUpload } from "element-plus";
import { ArrowRight } from "@element-plus/icons-vue";

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

const formResponce = ref({
  file_types: [],
  logic_habit: "",
  sort_struct: [],
  folder_depth: 5,
  capacity: 30,
  naming: [],
  archival_tendency: "",
});

const submitForm = () => {
  console.log("提交的表單資料:", formResponce.value);
  navigateTo("zone-wizard/CheckRule");
};
</script>

<template>
  <div class="container">
    <el-row justify="space-between" align="middle" :gutter="20">
      <el-col :span="22">
        <h1>Survey</h1>
      </el-col>
      <el-col :span="2">
        <el-button plain @click=""> Reset </el-button>
      </el-col>
    </el-row>
    <el-text class="mx-1" type="info">Some description</el-text>

    <file_types v-model="formResponce.file_types" />
    <logic_habit v-model="formResponce.logic_habit" />
    <sort_struct v-model="formResponce.sort_struct" />
    <folder_depth v-model="formResponce.folder_depth" />
    <capacity v-model="formResponce.capacity" />
    <naming_style v-model="formResponce.naming" />
    <archival_tendency v-model="formResponce.archival_tendency" />

    <!-- 跳轉介面的按鈕 -->
    <el-button type="primary" @click="submitForm">
      Submit
      <el-icon class="el-icon--right"><ArrowRight /></el-icon>
    </el-button>
  </div>
</template>

<style scoped>
.container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
  flex-wrap: wrap;
  text-wrap: normal;
}
</style>
