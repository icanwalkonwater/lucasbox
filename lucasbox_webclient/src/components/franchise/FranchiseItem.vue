<script setup lang="ts">
import { routeDetailFranchiseMovie, routeDetailFranchiseSerie } from "@/router";
import { ChevronRightIcon } from "vue-tabler-icons";
</script>

<template>
  <div class="p-2 md:p-5">
    <h2 class="text-3xl mb-4">Star Wars</h2>
    <p>Sunt excepteur sit proident eiusmod. Laboris sit dolor commodo laboris tempor aute labore veniam deserunt. Eu laborum deserunt incididunt dolor eu non elit consectetur. Proident Lorem ullamco esse duis ad mollit ullamco irure fugiat.</p>
  </div>
  <ul class="menu">
    <li>
      <RouterLink :to="{ name: routeDetailFranchiseMovie }" exact-active-class="active" class="flex justify-between">
        <span>Movies</span>
        <ChevronRightIcon />
      </RouterLink>
    </li>
    <li>
      <RouterLink :to="{ name: routeDetailFranchiseSerie }" exact-active-class="active" class="flex justify-between">
        <span>Serie</span>
        <ChevronRightIcon />
      </RouterLink>
    </li>
  </ul>
</template>
