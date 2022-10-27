<script setup lang="ts">
import PageLayout from "@/components/layout/PageLayout.vue";
import ListingMovieItem from "@/components/ListingMovieItem.vue";
import { routeDetailCollection } from "@/router";
import { useTestDataStore2 } from "@/stores/testData2";

const testData2 = useTestDataStore2();
</script>

<template>
  <PageLayout>
    <input type="text" placeholder="Search" class="input input-bordered input-md w-full" />
    
    <div class="grid grid-cols-3 gap-4 mt-5">

      <div v-for="(collection, i) in testData2.rootCollections" :key="i">
        <RouterLink
          :to="{ name: routeDetailCollection, params: { collectionId: collection.id } }" 
          custom
          v-slot="{ navigate }"
        >
          <ListingMovieItem
            :name="collection.name"
            :description="collection.description"
            @click="navigate"
          />
        </RouterLink>
      </div>
      
    </div>
  </PageLayout>
</template>
