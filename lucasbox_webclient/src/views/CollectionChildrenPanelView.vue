<script setup lang="ts">
import CollectionChildrenPanel from "@/components/collection/CollectionChildrenPanel.vue";
import { useRouteParams } from "@vueuse/router";
import { computed } from "vue";

const childIdRaw = useRouteParams<string>("childId");
const childId = computed(() => parseInt(childIdRaw.value));
</script>

<template>
  <CollectionChildrenPanel :child-id="childId" />
</template>
