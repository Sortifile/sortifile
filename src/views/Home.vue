<script setup>
import { useRouter } from "vue-router";
import { ElButton } from "element-plus";

const router = useRouter();

function navigateTo(page) {
  router.push(`/${page}`);
}
</script>

<template>
  <main class="container">
    <h1>Welcome to Sortifile</h1>

    <div class="grid">
      <el-button 
        class="button" 
        type="primary" 
        @click="navigateTo('create')">
        Create
      </el-button>
      <el-button 
        class="button" 
        type="primary" 
        @click="navigateTo('select')">
        Select
      </el-button>
      <el-button 
        class="button" 
        type="primary" 
        @click="navigateTo('settings')">
        Settings
      </el-button>
      <el-button 
        class="button" 
        type="primary" 
        @click="navigateTo('help')">
        Help
      </el-button>
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
