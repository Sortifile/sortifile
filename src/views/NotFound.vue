<template>
  <div class="not-found">
    <h1>404</h1>
    <p>Page Not Found</p>
    <router-link to="/">Go to Home</router-link>
  </div>
</template>

<script>
export default {
  name: "NotFound",
};
</script>

<style scoped>
.not-found {
  text-align: center;
  margin-top: 50px;
}

.not-found h1 {
  font-size: 100px;
  margin-bottom: 20px;
}

.not-found p {
  font-size: 24px;
  margin-bottom: 20px;
}

.not-found a {
  font-size: 18px;
  color: #42b983;
  text-decoration: none;
}
</style>
