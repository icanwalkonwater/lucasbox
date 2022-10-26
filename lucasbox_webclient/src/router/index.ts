import FranchiseTabMovies from "@/components/franchise/FranchiseTabMovies.vue";
import FranchiseRoot from "@/components/franchise/FranchiseRoot.vue";
import FranchiseTabSeries from "@/components/franchise/FranchiseTabSeries.vue";
import FranchiseView from "@/views/FranchiseView.vue";
import HomeView from "@/views/HomeView.vue";
import ListingView from "@/views/ListingView.vue";
import { createRouter, createWebHistory } from "vue-router";
import FranchiseMovie from "@/components/franchise/FranchiseMovie.vue";
import FranchiseSerie from "@/components/franchise/FranchiseSerie.vue";
import FranchiseSerieEpisode from "@/components/franchise/FranchiseSerieEpisode.vue";
import Empty from "@/components/ToDo.vue";

const NotFoundView = () => import("@/views/NotFoundView.vue");
const WipView = () => import("@/views/WipView.vue");

export const routeHome = Symbol();
export const routeListing = Symbol();
export const routeDetailCollection = Symbol();
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
      path: "/collection/:collectionId*",
      name: routeDetailCollection,
      component: FranchiseView,
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
