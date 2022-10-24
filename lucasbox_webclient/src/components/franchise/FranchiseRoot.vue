<script setup lang="ts">
import { useTestDataStore } from "@/stores/testData";
import { computed } from "vue";
import { useRouteParams } from "@vueuse/router";
import FranchiseItem from "./FranchiseItem.vue";

const testData = useTestDataStore();
const franchiseIdRaw = useRouteParams<string>("franchiseId");
const franchise = computed(() => {
  const fId = parseInt(franchiseIdRaw.value);
  return testData.franchises.find((f) => f.id === fId)!;
});
</script>

<template>
  <div class="grid grid-cols-2 h-full">
    <div class="h-full border-r">
      <FranchiseItem :franchise="franchise" />
    </div>
    <div>
      <RouterView />
    </div>
  </div>
</template>
