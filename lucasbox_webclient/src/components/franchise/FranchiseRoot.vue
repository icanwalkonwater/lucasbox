<script setup lang="ts">
import { useTestDataStore } from "@/stores/testData";
import { ref, watch, watchEffect } from "vue";
import { useRouteParams } from "@vueuse/router";
import FranchiseItem from "./FranchiseItem.vue";

const testData = useTestDataStore();
const franchiseId = useRouteParams("franchiseId");

const franchise = ref(testData.franchises[0]);

watchEffect(() => {
  franchise.value = testData.franchises.find((f) => f.id == route.params.franchiseId)!;
});

watch(() => route.params, (newParams) => {

});
</script>

<template>
  <div class="grid grid-cols-2 h-full">
    <div class="h-full border-r">
      <FranchiseItem />
    </div>
    <div>
      <RouterView />
    </div>
  </div>
</template>
