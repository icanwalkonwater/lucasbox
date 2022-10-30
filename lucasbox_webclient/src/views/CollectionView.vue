<script setup lang="ts">
import CollectionBody from "@/components/collection/CollectionBody.vue";
import PageLayout from "@/components/layout/PageLayout.vue";
import { route404 } from "@/router";
import { useCollectionById, useTestDataStore2 } from "@/stores/testData2";
import { useRouteParams } from "@vueuse/router";
import { computed, watchEffect } from "vue";
import { useRouter } from "vue-router";

const testData = useTestDataStore2();

const collectionIdRaw = useRouteParams<string>("collectionId");
const collectionId = computed(() => parseInt(collectionIdRaw.value));

const collection = useCollectionById(collectionId);

watchEffect(() => {
  if (collection.value === undefined) {
    useRouter().replace({ name: route404 });
  }
});
</script>

<template>
  <PageLayout class="!m-0 flex">
    <div class="grid grid-cols-2 flex-grow">
      <div class="border-r">
        <CollectionBody v-if="collection !== undefined" :collection="collection!" />
      </div>
      <div>
        <RouterView />
      </div>
    </div>
  </PageLayout>
</template>
