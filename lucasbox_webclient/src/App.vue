<script setup lang="ts">
import { provide } from "vue";
import { RouterView } from "vue-router";
import { DefaultApolloClient } from "@vue/apollo-composable";
import { apolloClient } from "./apollo";

provide(DefaultApolloClient, apolloClient);
</script>

<template>
  <RouterView />
</template>
