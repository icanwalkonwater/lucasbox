<script setup lang="ts">
import PageLayout from "@/components/layout/PageLayout.vue";
import { useRouteParams } from "@vueuse/router";

const collectionIds = useRouteParams<string | string[]>("collectionId");

console.log(typeof collectionIds.value,collectionIds.value.length, collectionIds.value);
</script>

<template>
  <PageLayout class="!m-0 flex">
    <div class="grid grid-cols-2 flex-grow">
      <div class="border-r">
        uwu
      </div>
      <div>
        owo
      </div>
    </div>
  </PageLayout>
</template>
