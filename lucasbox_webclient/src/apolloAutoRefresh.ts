// This is heavily based on this https://stackoverflow.com/a/63386965

import { fromPromise, type Operation } from "@apollo/client/core";
import { onError } from "@apollo/client/link/error";
import { assert } from "@vueuse/shared";
import { useAuthStore, useTokenRefreshStore } from "./stores/auth";

const tryRefreshToken = async (operation: Operation) => {
  const refreshStore = useTokenRefreshStore();

  const res = await refreshStore.requestRefreshAccessToken();

  // If no accessToken returned, well, at least we tried
  if (!res?.data?.refresh.accessToken) {
    throw Error("No access token");
  }

  const newAccesstoken = res.data.refresh.accessToken;

  const authStore = useAuthStore();
  assert(!!authStore.refreshToken);
  authStore.setTokens(newAccesstoken, authStore.refreshToken.value!);

  // Set the headers again for this operation
  const oldHeaders = operation.getContext().Headers;
  operation.setContext({
    headers: {
      ...oldHeaders,
      Authorization: `Bearer ${newAccesstoken}`,
    },
  });
};

export const tokenRefreshMiddleware = onError(({ graphQLErrors, forward, operation }) => {
  if (!graphQLErrors) {
    return;
  }

  for (const err of graphQLErrors) {
    // If unauthorized, try to get another refresh token and retry
    if (err.extensions?.code === "UNAUTHORIZED") {
      console.warn("Got unauthorized, trying to refresh token...");

      const authStore = useAuthStore();
      const refreshStore = useTokenRefreshStore();

      // If a token refresh is already in progress
      if (refreshStore.isTokenRefreshing) {
        console.log("Adding request to pending");
        refreshStore.appendPendingRequest(() => forward(operation));
        return;
      }

      // If a refresh token is present, continue
      if (authStore.refreshToken) {

        refreshStore.setIsTokenRefreshint(true);
        
        return fromPromise(tryRefreshToken(operation).catch(async () => {
          // If failed to refresh token, still execute pending requests to let them fail
          refreshStore.resolvePendingRequests();
          refreshStore.setIsTokenRefreshint(false);
          await authStore.logout();
        })).flatMap(() => {
          // Yay we are authenticated again, release the pending requests
          refreshStore.resolvePendingRequests();
          refreshStore.setIsTokenRefreshint(false);
          // Release the current request (its out of order but who cares)
          return forward(operation);
        });
      }
    }
  }
});
