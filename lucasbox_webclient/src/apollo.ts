import { createHttpLink, ApolloClient, InMemoryCache } from "@apollo/client";
import {  } from "@apollo/client/utilities";

const httpLink = createHttpLink({
    uri: "http://localhost:8080/graphql",
});

const cache = new InMemoryCache();

export const apolloClient = new ApolloClient({
    link: httpLink,
    cache,
});
