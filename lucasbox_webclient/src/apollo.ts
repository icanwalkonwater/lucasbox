import { createHttpLink, ApolloClient, InMemoryCache, ApolloLink, from } from "@apollo/client/core";
import { provideApolloClient } from "@vue/apollo-composable";
import { tokenRefreshMiddleware } from "./apolloAutoRefresh";
import { useAuthStore } from "./stores/auth";

export const httpLink = createHttpLink({
  uri: import.meta.env.VITE_GRAPHQL_ENDPOINT_URL,
});

const authMiddleware = new ApolloLink((operation, forward) => {
  const authStore = useAuthStore();

  if (authStore.isLoggedIn) {
    operation.setContext({
      headers: {
        Authorization: `Bearer ${authStore.accessToken}`,
      },
    });
  }

  return forward(operation);
});

export const apolloClient = new ApolloClient({
  link: from([tokenRefreshMiddleware, authMiddleware, httpLink]),
  cache: new InMemoryCache(),
});

provideApolloClient(apolloClient);
