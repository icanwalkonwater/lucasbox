<script setup lang="ts">
import CollectionBody from "@/components/collection/CollectionBody.vue";
import CollectionHeader from "@/components/collection/CollectionHeader.vue";
import PageLayout from "@/components/layout/PageLayout.vue";
import router, { route404 } from "@/router";
import { useCollection } from "@/stores/collections";
import { logErrorMessages } from "@vue/apollo-util";
import { useRouteParams } from "@vueuse/router";
import { computed, watchEffect } from "vue";
import { useRoute } from "vue-router";

const collectionIdRaw = useRouteParams<string>("collectionId");
const collectionId = computed(() => parseInt(collectionIdRaw.value));

const { result: queryRes, loading, onError } = useCollection(collectionId);
onError((err) => logErrorMessages(err));

const route = useRoute();
const needPanel = computed(() => !(route.meta?.noPanel ?? false));

watchEffect(() => {
  if (!loading.value && !queryRes.value?.collection) {
    router.replace({ name: route404 });
  }
});
</script>

<template>
  <PageLayout has-header>
    <template #header>
      <CollectionHeader v-if="!loading && queryRes!.collection !== null" :collection="queryRes!.collection!" />
      <span v-else>Loading...</span>
    </template>
    <template #default>
      <CollectionBody v-if="!loading && queryRes!.collection !== null" :collection="queryRes!.collection!" />
      <span v-else>Loading</span>
    </template>
  </PageLayout>
</template>
