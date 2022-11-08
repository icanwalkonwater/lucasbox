import { createHttpLink, ApolloClient, InMemoryCache, ApolloLink } from "@apollo/client/core";
import { tokenRefreshMiddleware } from "./apolloAutoRefresh";
import { useAuthStore } from "./stores/auth";

export const httpLink = createHttpLink({
  uri: import.meta.env.VITE_GRAPHQL_ENDPOINT_URL,
});

const authMiddleware = new ApolloLink((operation, forward) => {
  const authStore = useAuthStore();

  if (authStore.accessToken != undefined) {
    operation.setContext({
      Headers: {
        authorization: `Bearer ${authStore.accessToken}`,
      },
    });
  }

  return forward(operation);
});

export const apolloClient = new ApolloClient({
  link: tokenRefreshMiddleware.concat(authMiddleware.concat(httpLink)),
  cache: new InMemoryCache(),
});
