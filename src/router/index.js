import { createRouter, createWebHistory } from "vue-router";

// 自動載入 views 資料夾下的所有 .vue 檔案（包括子資料夾）
const modules = import.meta.glob("../views/**/*.vue");

// 動態生成路由（排除特殊路由，如 Home.vue 和 NotFound.vue）
const routes = Object.keys(modules)
  .filter(
    (path) => !path.includes("Home.vue") && !path.includes("NotFound.vue"),
  ) // 排除 Home 和 404 頁面
  .map((path) => {
    const name = path.match(/\/views\/(.*)\.vue$/)[1]; // 提取檔案路徑名稱
    const routePath = `/${name.toLowerCase()}`.replace(/\/index$/, ""); // 將路徑轉換為路由，移除 index
    return {
      path: routePath, // 對應路徑，例如 /abc 或 /folder/abc
      name: name.replace(/\//g, "-"), // 路由名稱，避免衝突
      component: modules[path], // 動態載入組件
    };
  });

// 添加特殊的根路由和 404 路由
routes.unshift(
  {
    path: "/",
    name: "Home",
    component: modules["../views/Home.vue"], // 指定 Home.vue 組件
  },
  {
    path: "/:pathMatch(.*)*", // 通配符路徑
    name: "NotFound",
    component: modules["../views/NotFound.vue"], // 指定 NotFound.vue 組件
  },
);

// 建立 Vue Router 實例
const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
