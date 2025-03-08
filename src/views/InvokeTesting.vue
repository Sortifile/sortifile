<template>
  <div>
    <h1>Invoke Testing</h1>
    <el-button class="button" @click="fun01()"> 01 </el-button>
    <el-button class="button" @click="fun02()"> 02 </el-button>
    <el-button class="button" @click="fun03()"> 03 </el-button>
    <el-button class="button" @click="fun04()"> 04 </el-button>
    <el-button class="button" @click="fun05()"> 05 </el-button>
    <el-button class="button" @click="fun06()"> 06 </el-button>
    <el-button class="button" @click="fun07()"> 07 </el-button>
    <el-button class="button" @click="fun08()"> 08 </el-button>
    <el-button class="button" @click="fun09()"> 09 </el-button>
    <el-button class="button" @click="fun10()"> 10 </el-button>
    <el-button class="button" @click="fun11()"> 11 </el-button>
    <el-button class="button" @click="fun12()"> 12 </el-button>
    <el-button class="button" @click="fun13()"> 13 </el-button>
    <el-button class="button" @click="fun14()"> 14 </el-button>
    <el-button class="button" @click="fun15()"> 15 </el-button>
    <el-button class="button" @click="fun16()"> 16 </el-button>
    <el-button class="button" @click="fun17()"> 17 </el-button>
    <el-button class="button" @click="fun18()"> 18 </el-button>
    <el-button class="button" @click="fun19()"> 19 </el-button>
    <el-button class="button" @click="fun20()"> 20 </el-button>
    <el-button class="button" @click="fun21()"> 21 </el-button>
    <pre>{{ returnContent }}</pre>
  </div>
</template>

<script setup>
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";
const returnContent = ref("");

function fun01() {
  invoke("ai_create_rule", {
    zoneName: "test",
    zonePath: "C:\\Users\\jimts\\ytp\\www",
    createFromStructure: false,
    formResponse: "hehe",
  }).then((response) => {
    returnContent.value = response;
  });
}

function fun02() {
  invoke("ai_summarize_all", {
    zoneName: "ttest",
    zonePath: "C:\\Users\\jimts\\ytp\\www",
    pathToSort: "",
  }).then((response) => {
    returnContent.value = response;
  });
}
function fun03() {
  invoke("create_zone", {
    zoneName: "ttest",
    rootPath: "C:\\Users\\jimts\\ytp\\www",
    config: "{}",
    ignore: "{}",
    rules: "{}",
  }).then((response) => {
    returnContent.value = response;
  });
}
function fun04() {
  invoke("ai_sort", {
    zoneName: "ttest",
    zonePath: "C:\\Users\\jimts\\ytp\\www",
    pathToSort: "",
  }).then((response) => {
    returnContent.value = response;
  });
}
function fun05() {
  invoke("get_file_tree", { zonePath: "C:\\Users\\jimts\\ytp\\tttttt" }).then(
    (response) => {
      returnContent.value = response;
    },
  );
}
function fun06() {}

function fun07() {
  invoke("get_file_tree", { zone_path: "test" }).then((response) => {
    returnContent.value = response;
  });
}

function fun08() {}
function fun09() {}
function fun10() {}
function fun11() {}
function fun12() {}
function fun13() {}
function fun14() {}
function fun15() {}
function fun16() {}
function fun17() {}
function fun18() {}
function fun19() {}
function fun20() {}
function fun21() {}
</script>
