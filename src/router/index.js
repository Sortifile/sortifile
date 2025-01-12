import { createRouter, createWebHistory } from "vue-router";

// Define routes
const routes = [
  {
    path: "/",
    name: "Home",
    component: () => import("../views/Home.vue"),
  },
  {
    path: "/about",
    name: "About",
    component: () => import("../views/About.vue"),
  },
  // Add a new route here if needed
];

// Create the router instance
const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
