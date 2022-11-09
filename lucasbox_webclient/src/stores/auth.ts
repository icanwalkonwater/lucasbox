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

const MUTATION_REFRESH = gql`
    mutation refresh($token: String!) {
        refresh(refreshToken: $token) {
            accessToken
        }
    }
`;

const MUTATION_LOGIN = gql`
    mutation login($username: String!, $password: String!) {
      login(username: $username, password: $password) {
        accessToken
        refreshToken
      }
    }
`;

const QUERY_ME = gql`
    query Me {
      me {
        username
      }
    }
`;

export const useLoginMutation = () => {
  return useMutation<{ login: { accessToken: string, refreshToken: string } }, { username: string, password: string }>(MUTATION_LOGIN);
};

export const useMeQuery = () => {
  return useQuery<{ username: string }>(QUERY_ME);
};

export const useAuthStore = defineStore("auth", () => {
  const accessToken = useStorage<string | null>(STORAGE_ACCESS_TOKEN_KEY, null);
  const refreshToken = useStorage<string | null>(STORAGE_REFRESH_TOKEN_KEY, null);

  const isLoggedIn = computed(() => !!(accessToken.value && refreshToken.value));

  const setTokens = (access: string, refresh: string) => {
    accessToken.value = access;
    refreshToken.value = refresh;
  };

  // Logout, clear everything
  const logout = async () => {
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
      mutation: MUTATION_REFRESH,
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
