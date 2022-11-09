<script setup lang="ts">
import { routeDetailCollectionChild } from "@/router";
import { useCollectionById } from "@/stores/testData2";
import { useArrayMap } from "@vueuse/shared";
import { toRefs } from "vue";
import { ChevronRightIcon } from "vue-tabler-icons";

const props = defineProps<{
  childrenIds: number[],
}>();

const { childrenIds } = toRefs(props);
const children = useArrayMap(childrenIds, (child) => useCollectionById(child).value);
</script>

<template>
  <ul class="menu">
    <li v-for="child in children" :key="child.id">
      <RouterLink :to="{ name: routeDetailCollectionChild, params: { childId: child.id } }" active-class="active" class="flex justify-between">
        {{ child.name }}
        <ChevronRightIcon />
      </RouterLink>
    </li>
  </ul>
</template>
