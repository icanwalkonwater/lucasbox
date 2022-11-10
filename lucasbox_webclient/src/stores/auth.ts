import { defineStore } from "pinia";
import { useStorage } from "@vueuse/core";
import gql from "graphql-tag";
import { computed, ref } from "vue";
import router, { routeLogin } from "@/router";
import { ApolloClient, InMemoryCache } from "@apollo/client/core";
import { apolloClient, httpLink } from "@/apollo";
import { useMutation, useQuery } from "@vue/apollo-composable";

export const STORAGE_ACCESS_TOKEN_KEY = "accessToken";
export const STORAGE_REFRESH_TOKEN_KEY = "refreshToken";

export const useLoginMutation = () => {
  return useMutation<{ login: { accessToken: string, refreshToken: string } }, { username: string, password: string }>(gql`
    mutation login($username: String!, $password: String!) {
      login(username: $username, password: $password) {
        accessToken
        refreshToken
      }
    }
  `);
};

export const useRegisterMutation = () => {
  return useMutation<{ register: { success: boolean } }, { username: string, password: string, inviteCode?: string }>(gql`
    mutation register($username: String!, $password: String!, $inviteCode: String) {
      register(username: $username, password: $password, inviteCode: $inviteCode) {
        success
      }
    }
  `);
};

export const useMeQuery = () => {
  return useQuery<{ username: string }>(gql`
    query Me {
      me {
        username
      }
    }
  `);
};

export const useAuthStore = defineStore("auth", () => {
  const accessToken = useStorage<string | null>(STORAGE_ACCESS_TOKEN_KEY, null, localStorage, { writeDefaults: false, listenToStorageChanges: true });
  const refreshToken = useStorage<string | null>(STORAGE_REFRESH_TOKEN_KEY, null, localStorage, { writeDefaults: false, listenToStorageChanges: true });

  const isLoggedIn = computed(() => !!(accessToken.value && refreshToken.value));

  const setTokens = (access: string, refresh: string) => {
    accessToken.value = access;
    refreshToken.value = refresh;
  };

  // Logout, clear everything
  const logout = async () => {
    accessToken.value = null;
    refreshToken.value = null;
    localStorage.clear();
    await apolloClient.cache.reset();

    await router.push({ name: routeLogin });
  };

  return { accessToken, refreshToken, isLoggedIn, setTokens, logout };
});

export const useTokenRefreshStore = defineStore("authTokenRefresh", () => {
  const authStore = useAuthStore();

  const isTokenRefreshing = ref(false);
  const pendingRequests = ref<Function[]>([]);

  const setIsTokenRefreshint = (isRefreshing: boolean) => {
    isTokenRefreshing.value = isRefreshing;
  };

  const appendPendingRequest = (req: Function) => {
    pendingRequests.value.push(req);
  };

  const requestRefreshAccessToken = async () => {
    const refreshClient = new ApolloClient({
      link: httpLink,
      cache: new InMemoryCache(),
    });

    return await refreshClient.mutate<{ refresh: { accessToken: string } }, { token: string }>({
      mutation: gql`
        mutation refresh($token: String!) {
            refresh(refreshToken: $token) {
                accessToken
            }
        }
      `,
      variables: {
        token: authStore.refreshToken ?? "",
      },
    });
  };

  const resolvePendingRequests = () => {
    pendingRequests.value.forEach((cb) => cb());
    pendingRequests.value = [];
  };

  return { isTokenRefreshing, pendingRequests, setIsTokenRefreshint, appendPendingRequest, requestRefreshAccessToken, resolvePendingRequests };
});
