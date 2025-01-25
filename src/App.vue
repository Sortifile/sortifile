<template>
  <div id="app" class="common-layout">
    <el-container direction="vertical">
      <Navbar />
      <el-main class="main-content">
        <router-view />
      </el-main>
      <Footer />
    </el-container>
  </div>
</template>

<script setup>
import Navbar from "./components/Navbar.vue";
import Footer from "./components/Footer.vue";
</script>

<style>
.common-layout {
  height: 100vh;
}

.el-container {
  height: 100%;
}

.main-content {
  padding-bottom: 60px !important;
  overflow-x: hidden;
  overflow-y: auto;
}

body {
  font-family: "Noto Sans TC", Arial, sans-serif;
  margin: 0;
}
</style>
