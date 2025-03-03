<template>
  <el-footer class="footer" height="40px">
    <p>© 2025 Sortifile</p>
  </el-footer>
</template>

<script>
export default {
  name: "Footer",
};
</script>

<style scoped>
/* 頁尾樣式 */
.footer {
  position: fixed;
  bottom: 0;
  left: 0;
  width: 100%;
  background-color: #394c68;
  color: white;
  text-align: right;
  align-content: center;
  box-sizing: border-box;
  padding: 0px 30px;
  z-index: 30;
}
.footer p {
  margin: 0;
}
</style>
