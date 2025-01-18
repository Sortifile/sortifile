<template>
  <div id="app" class="common-layout">
    <el-container>
      <el-header>
        <Navbar />
      </el-header>
      <el-main>
        <router-view />
      </el-main>
      <el-footer>
        <Footer />
      </el-footer>
    </el-container>
  </div>
</template>

<script setup>
import Navbar from "./components/Navbar.vue";
import Footer from "./components/Footer.vue";
</script>

<style>
body {
  font-family: "Noto Sans TC", Arial, sans-serif;
}
</style>
