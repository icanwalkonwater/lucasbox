<script setup lang="ts">
import { routeDetailCollectionChild } from "@/router";
import MenuLinkChevron from "../MenuLinkChevron.vue";

defineProps<{
  children: { id: number, name: string }[],
}>();
</script>

<template>
  <ul class="menu">
    <li v-for="child in children" :key="child.id">
      <MenuLinkChevron :route-name="routeDetailCollectionChild" :params="{ childId: child.id }">
        {{ child.name }}
      </MenuLinkChevron>
    </li>
  </ul>
</template>
