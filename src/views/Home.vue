<script setup>
import { useRouter } from "vue-router";
import { ElButton } from "element-plus";
import { invoke } from "@tauri-apps/api/core";
import { onMounted } from "vue";
import { useZoneStore } from "../store/zone";
import { useRuleStore } from "../store/rule";
import { useFormStore } from "../store/form";

const zoneStore = useZoneStore();
const ruleStore = useRuleStore();
const formStore = useFormStore();
const router = useRouter();

function navigateTo(page) {
  router.push(`/${page}`);
}

function goToGithub() {
  window.open("https://github.com/sortifile/sortifile");
}

onMounted(() => {
  console.log("Home page mounted");
  // clear all store data
  zoneStore.resetZone();
  ruleStore.resetRule();
  formStore.resetForm();
});
</script>

<template>
  <main class="container">
    <img src="/sortifile.png" alt="sortifile logo" />
    <div class="grid">
      <el-tooltip content="New Zone..." placement="left-start">
        <el-button class="button" type="primary" @click="navigateTo('create')">
          Create
        </el-button>
      </el-tooltip>
      <el-tooltip placement="right-start">
        <template #content> Select Zone<br />and See Zone Details </template>
        <el-button class="button" type="primary" @click="navigateTo('select')">
          Select
        </el-button>
      </el-tooltip>
      <el-button class="button" type="primary" @click="navigateTo('settings')">
        Settings
      </el-button>
      <!-- <el-button class="button" type="primary" @click="goToGithub">
        Docs
      </el-button> -->
      <el-button
        class="button"
        type="primary"
        @click="navigateTo('invoketesting')"
      >
        invoke
      </el-button>
    </div>
  </main>
</template>

<style scoped>
img {
  height: 30vh;
  margin-top: 20px;
  margin-bottom: 20px;
}

.container {
  height: 100%;
  width: 100%;
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;
}

.grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 20px;
  width: 300px;
}

.button {
  font-size: 1.2em;
  font-weight: bold;
  width: 100%;
  height: 100px;
  display: flex;
  justify-content: center;
  align-items: center;
  border-radius: 8px;
  margin: 0;
}

h1 {
  margin-bottom: 20px;
}
</style>
