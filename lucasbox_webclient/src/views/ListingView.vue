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

      <div v-for="(collection, i) in testData2.collections" :key="i">
        <RouterLink
          :to="{ name: routeDetailCollection, params: { franchiseId: collection.id } }" 
          custom
          v-slot="{ navigate }"
        >
          <ListingMovieItem
            :name="franchise.title"
            :description="franchise.description"
            :movies="franchise.movies?.length ?? 0"
            :series="franchise.series?.length ?? 0"
            @click="navigate"
          />
        </RouterLink>
      </div>
      
    </div>
  </PageLayout>
</template>
