<template>
  <nav class="navbar">
    <ul>
      <li><router-link to="/">Home</router-link></li>
      <li><router-link to="/about">About</router-link></li>
    </ul>
  </nav>
</template>

<script>
export default {
  name: "Navbar",
};
</script>

<style scoped>
/* 導覽列樣式 */
.navbar {
  display: flex;
  justify-content: flex-end; /* 內容靠右對齊 */
  padding: 10px 20px; /* 增加上下和左右內距 */
  border-bottom: 1px solid #cfcfcf; /* 下方添加淺灰色線條 */
}

.navbar ul {
  list-style: none; /* 移除列表項的圓點樣式 */
  display: flex; /* 橫向排列 */
  margin: 0; /* 移除外邊距 */
  padding: 0; /* 移除內邊距 */
}

.navbar li {
  margin-left: 30px; /* 每個項目之間留間距 */
}

.navbar a {
  text-decoration: none; /* 移除連結的底線 */
  color: #333; /* 預設文字顏色 */
  font-size: 1em;
  font-weight: 500;
  transition: color 0.3s; /* 加入平滑變色效果 */
}

.navbar a:hover {
  color: #007bff; /* 滑鼠懸停變為藍色 */
}
</style>
