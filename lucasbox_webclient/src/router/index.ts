import ToDoVue from "@/components/ToDo.vue";
import CollectionView from "@/views/CollectionView.vue";
import HomeView from "@/views/HomeView.vue";
import ListingView from "@/views/ListingView.vue";
import { createRouter, createWebHistory } from "vue-router";

const NotFoundView = () => import("@/views/NotFoundView.vue");
const WipView = () => import("@/views/WipView.vue");

export const routeHome = Symbol();
export const routeListing = Symbol();
export const routeDetailCollection = Symbol();
export const routeDetailCollectionChild = Symbol();
export const routeDetailCollectionItem = Symbol();
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
      path: "/collection/:collectionId(\\d+)",
      name: routeDetailCollection,
      component: CollectionView,
      children: [
        {
          path: "c/:childId(\\d+)",
          name: routeDetailCollectionChild,
          component: ToDoVue,
        },
        {
          path: "i/:itemId(\\d+)",
          name: routeDetailCollectionItem,
          component: ToDoVue,
        }
      ]
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
