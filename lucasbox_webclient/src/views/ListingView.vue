<script setup lang="ts">
import PageLayout from "@/components/layout/PageLayout.vue";
import ListingMovieItem from "@/components/ListingMovieItem.vue";
import { routeDetailFranchise } from "@/router";

const movieData = {
  name: "Le seigneur de l'anal",
  description: "uwu legolas-chan",
  movies: 6,
  series: 1,
};
</script>

<template>
  <PageLayout>
    <input type="text" placeholder="Search" class="input input-bordered input-md w-full" />
    
    <div class="grid grid-cols-3 gap-4 mt-5">
      <div v-for="i in 10" :key="i">
        <RouterLink :to="{ name: routeDetailFranchise, params: { fid: 1 } }" custom v-slot="{ navigate }">
          <ListingMovieItem
            v-bind="movieData"
            @click="navigate"
          />
        </RouterLink>
      </div>
    </div>
  </PageLayout>
</template>
