import FranchiseTabMovies from "@/components/franchise/FranchiseTabMovies.vue";
import FranchiseRoot from "@/components/franchise/FranchiseRoot.vue";
import FranchiseTabSeries from "@/components/franchise/FranchiseTabSeries.vue";
import FranchiseView from "@/views/FranchiseView.vue";
import HomeView from "@/views/HomeView.vue";
import ListingView from "@/views/ListingView.vue";
import { createRouter, createWebHistory } from "vue-router";
import FranchiseMovie from "@/components/franchise/FranchiseMovie.vue";

const NotFoundView = () => import("@/views/NotFoundView.vue");
const WipView = () => import("@/views/WipView.vue");

export const routeHome = Symbol();
export const routeListing = Symbol();
export const routeDetailFranchise = Symbol();
export const routeDetailFranchiseMovie = Symbol();
export const routeDetailFranchiseSerie = Symbol();
export const routeDetailMovie = Symbol();
export const routeDetailSerie = Symbol();
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
      path: "/listing",
      name: routeListing,
      component: ListingView,
    },
    {
      path: "/franchise/:fid",
      component: FranchiseView,
      children: [
        {
          path: "",
          name: routeDetailFranchise,
          component: FranchiseRoot,
          children: [
            {
              path: "movies",
              name: routeDetailFranchiseMovie,
              component: FranchiseTabMovies,
            },
            {
              path: "series",
              name: routeDetailFranchiseSerie,
              component: FranchiseTabSeries,
            },
          ],
        },
        {
          path: "movies/:mid",
          name: routeDetailMovie,
          component: FranchiseMovie,
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
