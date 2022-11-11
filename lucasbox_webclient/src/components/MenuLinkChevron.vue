<script setup lang="ts">
import type { RouteParamsRaw, RouteRecordName } from "vue-router";
import { ChevronRightIcon } from "vue-tabler-icons";

defineProps<{
    routeName: RouteRecordName,
    params?: RouteParamsRaw,
}>();
</script>

<template>
  <RouterLink
    :to="{ name: routeName, params }"
    active-class="active"
    class="flex justify-between"
  >
    <slot></slot>
    <ChevronRightIcon />
  </RouterLink>
</template>
