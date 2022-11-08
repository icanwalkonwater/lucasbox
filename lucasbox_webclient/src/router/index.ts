import CollectionChildrenPanel from "@/components/collection/CollectionChildrenPanel.vue";
import ToDo from "@/components/ToDo.vue";
import CollectionView from "@/views/CollectionView.vue";
import HomeView from "@/views/HomeView.vue";
import ListingView from "@/views/ListingView.vue";
import LoginView from "@/views/LoginView.vue";
import { createRouter, createWebHistory } from "vue-router";

const NotFoundView = () => import("@/views/NotFoundView.vue");
const WipView = () => import("@/views/WipView.vue");

export const routeHome = Symbol();
export const routeLogin = Symbol();
export const routeRegister = Symbol();
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
      path: '/login',
      name: routeLogin,
      component: LoginView,
      meta: {
        noAuthRequired: true,
      },
    },
    {
      path: '/register',
      name: routeRegister,
      component: ToDo,
      meta: {
        noAuthRequired: true,
      },
    },
    {
      path: "/listing",
      name: routeListing,
      component: ListingView,
    },
    {
      path: "/collection/:collectionId(\\d+)",
      component: CollectionView,
      children: [
        {
          path: "",
          name: routeDetailCollection,
          component: ToDo,
          meta: {
            noPanel: true,
          },
        },
        {
          path: "c/:childId(\\d+)",
          name: routeDetailCollectionChild,
          component: CollectionChildrenPanel,
        },
        {
          path: "i/:itemId(\\d+)",
          name: routeDetailCollectionItem,
          component: ToDo,
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
      path: "/404",
      name: route404,
      component: NotFoundView,
    },
    {
      path: "/:p(.*)*",
      component: NotFoundView,
    },
  ],
});

router.beforeEach((to) => {
  if (!(to.meta.noAuthRequired ?? false)) {
    return { name: routeLogin };
  }
});

export default router;
