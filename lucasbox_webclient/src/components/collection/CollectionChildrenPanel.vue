<script setup lang="ts">
import { useTestDataStore2 } from "@/stores/testData2";
import { useRouteParams } from "@vueuse/router";
import { computed } from "vue";

const testData = useTestDataStore2();

const childIdRaw = useRouteParams<string>("childId");
const childId = computed(() => parseInt(childIdRaw.value));

const collection = computed(() => testData.collections.find(({ id }) => id === childId.value));
</script>

<template>
  {{ collection }}
</template>
