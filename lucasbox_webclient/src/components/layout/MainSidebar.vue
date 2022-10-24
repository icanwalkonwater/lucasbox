<script setup lang="ts">
import { routeUpload, routeHome, routeListing, routeSettings } from "@/router";
import { HomeIcon, MovieIcon, UploadIcon, SettingsIcon } from "vue-tabler-icons";
</script>

<template>
  <div class="sticky top-0 h-screen border-r">
    <div class="flex flex-col">
      <!-- Sidebar top icon -->
      <button class="btn btn-ghost rounded-none">
        <img class="h-7 w-7" src="/favicon.ico">
      </button>
      <div class="h-10"></div>
      <ul class="menu">
        <li>
          <RouterLink :to="{ name: routeHome }" exact-active-class="active" class="place-content-center">
            <HomeIcon />
          </RouterLink>
        </li>
        <li>
          <RouterLink :to="{ name: routeListing }" active-class="active" class="place-content-center">
            <MovieIcon />
          </RouterLink>
        </li>
        <li>
          <RouterLink :to="{ name: routeUpload }" active-class="active" class="place-content-center">
            <UploadIcon />
          </RouterLink>
        </li>
        <li>
          <RouterLink :to="{ name: routeSettings }" active-class="active" class="place-content-center">
            <SettingsIcon />
          </RouterLink>
        </li>
      </ul>
    </div>
  </div>
</template>
