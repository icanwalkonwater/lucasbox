import CollectionChildrenPanelView from "@/views/CollectionChildrenPanelView.vue";
import ToDo from "@/components/ToDo.vue";
import { useAuthStore } from "@/stores/auth";
import CollectionView from "@/views/CollectionView.vue";
import HomeView from "@/views/HomeView.vue";
import ListingView from "@/views/ListingView.vue";
import LoginView from "@/views/LoginView.vue";
import RegisterView from "@/views/RegisterView.vue";
import { createRouter, createWebHistory } from "vue-router";
import CollectionBodyNewView from "@/views/CollectionBodyNewView.vue";

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
      path: "/login",
      name: routeLogin,
      component: LoginView,
      meta: {
        authWorkflow: true,
      },
    },
    {
      path: "/register",
      name: routeRegister,
      component: RegisterView,
      meta: {
        authWorkflow: true,
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
          component: CollectionChildrenPanelView,
        },
        {
          path: "i/:itemId(\\d+)",
          name: routeDetailCollectionItem,
          component: ToDo,
        },
      ],
    },
    {
      path: "/collection_new",
      component: CollectionBodyNewView,
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
  const authStore = useAuthStore();

  if (to.meta.authWorkflow) {
    // If part of the auth workflow and already connected, get out of here
    if (authStore.isLoggedIn) {
      return { name: routeHome };
    }
  } else {
    // If not part of the workflow and no auth, go connect yourself
    if (!authStore.isLoggedIn) {
      return { name: routeLogin };
    }
  }
});

export default router;
