import { defineStore } from "pinia";
import { useStorage } from "@vueuse/core";
import gql from "graphql-tag";
import { ref } from "vue";
import { useRouter } from "vue-router";
import { routeLogin } from "@/router";
import { ApolloClient, InMemoryCache } from "@apollo/client/core";
import { httpLink } from "@/apollo";

export const STORAGE_ACCESS_TOKEN_KEY = "accessToken";
export const STORAGE_REFRESH_TOKEN_KEY = "refreshToken";

const MUTATION_REFRESH = gql`
    mutation refresh($token: string!) {
        refresh(refreshToken: $token) {
            accessToken
        }
    }
`;

const MUTATION_LOGIN = gql`
    mutation login($username: string!, $password: string!) {
      login(username: $username, password: $password) {
        accessToken
        refreshToken
      }
    }
`;

export const useAuthStore = defineStore("auth", () => {
  const accessToken = useStorage<string | undefined>(STORAGE_ACCESS_TOKEN_KEY, undefined);
  const refreshToken = useStorage<string | undefined>(STORAGE_REFRESH_TOKEN_KEY, undefined);

  const logout = () => {
    accessToken.value = undefined;
    refreshToken.value = undefined;

    const router = useRouter();
    router.push({ name: routeLogin });
  };

  return { accessToken, refreshToken, logout };
});

export const useTokenRefreshStore = defineStore("authTokenRefresh", () => {
  const authStore = useAuthStore();

  const isTokenRefreshing = ref(false);
  const pendingRequests = ref<Function[]>([]);

  const requestRefreshAccessToken = async () => {
    const refreshClient = new ApolloClient({
      link: httpLink,
      cache: new InMemoryCache(),
    });

    return await refreshClient.mutate<{ accessToken?: string }, { token: string }>({
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

  return { isTokenRefreshing, pendingRequests, requestRefreshAccessToken, resolvePendingRequests };
});
