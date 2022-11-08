<script setup lang="ts">
import CollectionBody from "@/components/collection/CollectionBody.vue";
import PageLayout from "@/components/layout/PageLayout.vue";
import { route404 } from "@/router";
import { useCollectionById, useTestDataStore2 } from "@/stores/testData2";
import { useRouteParams } from "@vueuse/router";
import { computed, watchEffect } from "vue";
import { useRoute, useRouter } from "vue-router";

const testData = useTestDataStore2();

const collectionIdRaw = useRouteParams<string>("collectionId");
const collectionId = computed(() => parseInt(collectionIdRaw.value));

const collection = useCollectionById(collectionId);

const route = useRoute();
const needPanel = computed(() => !(route.meta?.noPanel ?? false));

watchEffect(() => {
  if (collection.value === undefined) {
    useRouter().replace({ name: route404 });
  }
});
</script>

<template>
  <PageLayout class="!m-0 flex">
    <div :class="{ 'grid': true, 'flex-grow': true, 'grid-cols-2': needPanel }">
      <div class="border-r">
        <CollectionBody v-if="collection !== undefined" :collection="collection!" />
      </div>
      <div v-if="needPanel">
        <RouterView />
      </div>
    </div>
  </PageLayout>
</template>
