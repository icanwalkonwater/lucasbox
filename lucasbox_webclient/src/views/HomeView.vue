<script setup lang="ts">
import PageLayout from "../components/layout/PageLayout.vue";
import StatItem from "../components/StatItem.vue";
import FeaturedMovieItem from "../components/FeaturedMovieItem.vue";
import { DownloadIcon } from "vue-tabler-icons";
import { useTestDataStore2 } from "@/stores/testData2";

const testData2 = useTestDataStore2();

const movie = {
  title: "TODO",
  description: "TODOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO",
  badges: ["TODO"],
};
</script>

<template>
  <PageLayout>
    <div class="grid grid-cols-1 md:grid-cols-4 gap-2 md:gap-4">
      <StatItem
        v-for="(stat, index) in testData2.indicators"
        :key="index"
        :title="stat.name"
        :value="stat.value"
        :percent="stat?.percent"
        :critical="stat?.critical"
      />
    </div>

    <div class="divider"></div>

    <div class="grid grid-cols-1 md:grid-cols-3 md:gap-4">
      <FeaturedMovieItem v-bind="movie">
        <template v-slot:featured>
          <span class="badge badge-info"><DownloadIcon size="16" />&nbsp;Most downloaded</span>
        </template>
      </FeaturedMovieItem>
    </div>
  </PageLayout>
</template>
