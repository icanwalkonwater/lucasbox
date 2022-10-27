<script setup lang="ts">
import { routeDetailCollectionChild } from "@/router";
import { useTestDataStore2 } from "@/stores/testData2";
import { computed } from "vue";
import { ChevronRightIcon } from "vue-tabler-icons";

const testData = useTestDataStore2();

const props = defineProps<{
  childrenIds: number[],
}>();

const children = computed(() => props.childrenIds.map((child) => testData.collections.find(({ id }) => id === child)!));
</script>

<template>
  <ul class="menu">
    <li v-for="child in children" :key="child.id">
      <RouterLink :to="{ name: routeDetailCollectionChild, params: { childId: child.id } }" class="flex justify-between">
        {{ child.name }}
        <ChevronRightIcon />
      </RouterLink>
    </li>
  </ul>
</template>
