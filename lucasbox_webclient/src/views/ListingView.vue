<script setup lang="ts">
import PageLayout from "@/components/layout/PageLayout.vue";
import ListingMovieItem from "@/components/ListingMovieItem.vue";
import { routeDetailCollection } from "@/router";
import { useQueryRootCollections } from "@/stores/collections";
import { logErrorMessages } from "@vue/apollo-util";

const { result: queryRes, loading, onError } = useQueryRootCollections();
onError((err) => logErrorMessages(err));
</script>

<template>
  <PageLayout>
    <input
      type="text"
      placeholder="Search"
      class="input input-bordered input-md w-full"
    />

    <div class="grid grid-cols-3 gap-4 mt-5">
      <template v-if="!loading">
        <div v-for="collection in queryRes!.rootCollections" :key="collection.id">
          <RouterLink
            :to="{ name: routeDetailCollection, params: { collectionId: collection.id } }"
            custom
            v-slot="{ navigate }"
          >
            <ListingMovieItem
              :name="collection.name"
              :description="collection.description"
              :tags="collection.tags.map(({ label }) => label)"
              @click="navigate"
            />
          </RouterLink>
        </div>
      </template>
    </div>
  </PageLayout>
</template>
