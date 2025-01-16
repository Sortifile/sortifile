<script setup>
import { useRouter } from "vue-router";

const router = useRouter();

function navigateTo(page) {
  router.push(`/${page}`); // 根據你的 router 路徑生成規則
}
</script>

<template>
  <main class="container">
    <h1>Welcome to Sortifile</h1>

    <div class="grid">
      <div class="box" @click="navigateTo('create')">Create</div>
      <div class="box" @click="navigateTo('select')">Select</div>
      <div class="box" @click="navigateTo('settings')">Settings</div>
      <div class="box" @click="navigateTo('help')">Help</div>
    </div>
  </main>
</template>

<style scoped>
.container {
  margin: 0;
  padding-top: 10vh;
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

.box {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100px;
  background-color: #647eff;
  color: white;
  font-size: 1.2em;
  font-weight: bold;
  border-radius: 8px;
  cursor: pointer;
  transition:
    background-color 0.3s,
    transform 0.2s;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.box:hover {
  background-color: #535bf2;
  transform: translateY(-5px);
}

.box:active {
  background-color: #3a48a2;
  transform: translateY(0);
}

h1 {
  margin-bottom: 20px;
}
</style>
