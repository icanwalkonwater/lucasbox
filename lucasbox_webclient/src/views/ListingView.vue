<script setup lang="ts">
import PageLayout from "@/components/layout/PageLayout.vue";
import ListingMovieItem from "@/components/ListingMovieItem.vue";
import { routeDetailFranchise } from "@/router";
import { useTestDataStore } from "@/stores/testData";

const testData = useTestDataStore();
</script>

<template>
  <PageLayout>
    <input type="text" placeholder="Search" class="input input-bordered input-md w-full" />
    
    <div class="grid grid-cols-3 gap-4 mt-5">

      <div v-for="(franchise, i) in testData.franchises" :key="i">
        <RouterLink
          :to="{ name: routeDetailFranchise, params: { franchiseId: franchise.id } }" 
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
