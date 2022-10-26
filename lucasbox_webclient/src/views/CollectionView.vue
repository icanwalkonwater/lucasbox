<script setup lang="ts">
import PageLayout from "@/components/layout/PageLayout.vue";
import { useTestDataStore2 } from "@/stores/testData2";
import { useRouteParams } from "@vueuse/router";
import { computed } from "vue";

const testData = useTestDataStore2();

const collectionIdRaw = useRouteParams<string>("collectionId");
const collectionId = computed(() => parseInt(collectionIdRaw.value));

const collection = computed(() => testData.collections.find(({ id }) => id === collectionId.value)!);
</script>

<template>
  <PageLayout class="!m-0 flex">
    <div class="grid grid-cols-2 flex-grow">
      <div class="border-r">
        {{ collection }}
      </div>
      <div>
        <RouterView>
          Select something
          </RouterView>
      </div>
    </div>
  </PageLayout>
</template>
