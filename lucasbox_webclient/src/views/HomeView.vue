<script setup lang="ts">
import HomeLayout from '@/components/layout/PageLayout.vue';
import StatItem from '../components/StatItem.vue';
import { DownloadIcon } from "vue-tabler-icons";

const statData = [
  {
    name: "Uptime",
    value: "35 days",
  },
  {
    name: "CPU",
    value: "i9-9999Z",
    percent: 12,
  },
  {
    name: "RAM",
    value: "3.4/4 Go",
    percent: 85,
  },
  {
    name: "Used disk space",
    value: "3.9/4 To",
    percent: 97,
    critical: true,
  },
  {
    name: "Active downloads",
    value: "5",
  },
  {
    name: "Network usage down",
    value: "1000 Mbps",
  },
];
</script>

<template>
  <HomeLayout>
    <div class="grid grid-cols-1 md:grid-cols-4 gap-2 md:gap-4">
      <StatItem
        v-for="(stat, index) in statData"
        :key="index"
        :title="stat.name"
        :value="stat.value"
        :percent="stat?.percent"
        :critical="stat?.critical"
      />
    </div>

    <div class="divider"></div>

    <div class="grid grid-cols-1 md:grid-cols-3 md:gap-4">
      <div class="card border image-full">
        <figure><img src="https://placeimg.com/640/480/nature"></figure>
        <div class="card-body">
          <div class="card-title">Le seigneur de l'anal</div>
          <div class="flex gap-1">
            <span class="badge badge-info"><DownloadIcon size="16" />&nbsp;Most downloaded</span>
            <span class="badge">Movie</span>
            <span class="badge">Trilogie</span>
            <span class="badge">9h</span>
          </div>
          <p>Pariatur reprehenderit aliqua quis dolore et ad adipisicing irure enim ad proident est.</p>
          <div class="card-actions justify-end">
            <button class="btn btn-primary">Watch</button>
          </div>
        </div>
      </div>
    </div>
  </HomeLayout>
</template>
