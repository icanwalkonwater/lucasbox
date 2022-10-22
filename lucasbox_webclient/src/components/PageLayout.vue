<script setup lang="ts">
import { HomeIcon, MovieIcon, UploadIcon, SettingsIcon } from 'vue-tabler-icons';
import StatItem from './StatItem.vue';

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
  <div class="flex">
    <!-- Sidebar -->
    <div class="h-screen border-r">
      <div class="flex flex-col">
        <!-- Sidebar top icon -->
        <button class="btn btn-ghost btn rounded-none">
          <img class="h-7 w-7" src="/favicon.ico">
        </button>
        <div class="h-10"></div>
        <ul class="menu">
          <li>
            <a class="active place-content-center">
              <HomeIcon />
            </a>
          </li>
          <li>
            <a class="place-content-center">
              <MovieIcon />
            </a>
          </li>
          <li>
            <a class="place-content-center">
              <UploadIcon />
            </a>
          </li>
          <li>
            <a class="place-content-center">
              <SettingsIcon />
            </a>
          </li>
        </ul>
      </div>
    </div>

    <!-- Main block -->
    <div class="w-full overflow-y-scroll">
      
      <!-- Navbar -->
      <nav class="navbar border-b">
        <div class="navbar-start">
          <h1 class="text-lg font-bold mx-5">LucasFilms</h1>
          <div class="w-5"></div>
          <div class="breadcrumbs">
            <ul>
              <li><a>Home</a></li>
              <li><a>Stats</a></li>
            </ul>
          </div>
        </div>
      </nav>
      
      <main class="m-2 md:m-5">
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
      </main>
    
    </div>
  </div>
</template>