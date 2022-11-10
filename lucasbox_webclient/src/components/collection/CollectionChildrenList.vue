<script setup lang="ts">
import { routeDetailCollectionChild } from "@/router";
import { ChevronRightIcon } from "vue-tabler-icons";

defineProps<{
  children: { id: number, name: string }[],
}>();
</script>

<template>
  <ul class="menu">
    <li v-for="child in children" :key="child.id">
      <RouterLink
        :to="{ name: routeDetailCollectionChild, params: { childId: child.id } }"
        active-class="active"
        class="flex justify-between"
      >
        {{ child.name }}
        <ChevronRightIcon />
      </RouterLink>
    </li>
  </ul>
</template>
