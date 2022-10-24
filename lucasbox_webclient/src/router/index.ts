import FranchiseViewVue from "@/views/FranchiseView.vue";
import HomeView from "@/views/HomeView.vue";
import ListingView from "@/views/ListingView.vue";
import { createRouter, createWebHistory } from "vue-router";

const NotFoundView = () => import("@/views/NotFoundView.vue");
const WipView = () => import("@/views/WipView.vue");

export const routeHome = Symbol();
export const routeListing = Symbol();
export const routeDetailFranchise = Symbol();
export const routeDetailFranchiseSerie = Symbol();
export const routeDetailFranchiseSerieEpisode = Symbol();
export const routeDetailFranchiseMovie = Symbol();
export const routeUpload = Symbol();
export const routeSettings = Symbol();
export const route404 = Symbol();

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: routeHome,
      component: HomeView,
    },
    {
      path: "/list",
      name: routeListing,
      component: ListingView,
    },
    {
      path: "/f/:fid",
      name: routeDetailFranchise,
      component: FranchiseViewVue,
      children: [
        {
          path: "m/:mid",
          name: routeDetailFranchiseMovie,
          component: WipView,
        },
        {
          path: "s/:sid",
          name: routeDetailFranchiseSerie,
          component: WipView,
          children: [
            {
              path: ":seid",
              name: routeDetailFranchiseSerieEpisode,
              component: WipView,
            },
          ],
        },
      ],
    },
    {
      path: "/upload",
      name: routeUpload,
      component: WipView,
    },
    {
      path: "/settings",
      name: routeSettings,
      component: WipView,
    },
    {
      path: "/:pathMatch(.*)*",
      name: route404,
      component: NotFoundView,
    },
  ],
});

export default router;
