<script setup lang="ts">
import { routeUpload, routeHome, routeListing, routeSettings } from "@/router";
import { useAuthStore } from "@/stores/auth";
import { HomeIcon, UploadIcon, SettingsIcon, SearchIcon, LogoutIcon } from "vue-tabler-icons";

const authStore = useAuthStore();
</script>

<template>
  <div class="sticky top-0 h-screen border-r">
    <div class="flex flex-col">
      <!-- Sidebar top icon -->
      <RouterLink :to="{ name: routeHome }" custom v-slot="{ navigate }">
        <button class="btn btn-ghost rounded-none" @click="navigate">
          <img class="h-7 w-7" src="/favicon.ico">
        </button>
      </RouterLink>
      <div class="h-10"></div>
      <ul class="menu">
        <li>
          <RouterLink :to="{ name: routeHome }" exact-active-class="active" class="place-content-center">
            <HomeIcon />
          </RouterLink>
        </li>
        <li>
          <RouterLink :to="{ name: routeListing }" active-class="active" class="place-content-center">
            <SearchIcon />
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

      <div class="divider"></div>

      <ul class="menu">
        <li>
          <a class="place-content-center" @click="authStore.logout">
            <LogoutIcon class="ml-1" />
          </a>
        </li>
      </ul>
    </div>
  </div>
</template>
