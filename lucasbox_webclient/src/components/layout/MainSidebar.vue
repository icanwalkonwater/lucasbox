<script setup lang="ts">
import { HomeIcon, MovieIcon, UploadIcon, SettingsIcon } from "vue-tabler-icons";
</script>

<template>
  <div class="h-screen border-r">
    <div class="flex flex-col">
      <!-- Sidebar top icon -->
      <button class="btn btn-ghost rounded-none">
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
</template>