<script setup lang="ts">
import { route404 } from "@/router";
import { ChevronRightIcon } from "vue-tabler-icons";
import type { Franchise } from "@/stores/testData";
import { useRouter } from "vue-router";
import { computed, watchEffect } from "vue";

const props = defineProps<{ franchise: Franchise }>();
const router = useRouter();

const isMovieEmpty = computed(() => (props.franchise.movies?.length ?? 0) === 0);
const isSeriesEmpty = computed(() => (props.franchise.series?.length ?? 0) === 0);
const isExtraEmpty = computed(() => (props.franchise.extra?.length ?? 0) === 0);

const isSingleMovie = computed(() => props.franchise.movies?.length === 1 && isSeriesEmpty && isExtraEmpty);
const isSingleSerie = computed(() => props.franchise.series?.length === 1 && isMovieEmpty && isExtraEmpty);

/*
watchEffect(() => {
  if (isSingleMovie.value) {
    router.replace({ name: routeDetailMovie, params: { movieId: props.franchise.movies![0].id } });
  }
});*/
</script>

<template>
  <div class="p-2 md:p-5">
    <h2 class="text-3xl mb-4">{{ franchise.title }}</h2>
    <p>{{ franchise.description }}</p>
  </div>
  <ul class="menu">
    <li>
      <RouterLink :to="{ name: route404 }" exact-active-class="active" class="flex justify-between">
        <span>Movies</span>
        <ChevronRightIcon />
      </RouterLink>
    </li>
    <li>
      <RouterLink :to="{ name: route404 }" exact-active-class="active" class="flex justify-between">
        <span>Serie</span>
        <ChevronRightIcon />
      </RouterLink>
    </li>
    <li><a class="flex justify-between">
      <span>Extra (TODO)</span>
      <ChevronRightIcon />
    </a></li>
  </ul>
</template>
