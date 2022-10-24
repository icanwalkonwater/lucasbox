import HomeView from "@/views/HomeView.vue";
import ListingView from "@/views/ListingView.vue";
import WipView from "@/views/WipView.vue";
import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: HomeView,
    },
    {
      path: "/list",
      name: "list",
      component: ListingView,
    },
    {
      path: "/upload",
      name: "upload",
      component: WipView,
    },
    {
      path: "/settings",
      name: "settings",
      component: WipView,
    },
  ],
});

export default router;
