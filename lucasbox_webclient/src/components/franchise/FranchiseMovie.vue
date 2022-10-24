<script setup lang="ts">
import { routeDetailMovie } from "@/router";
import { ChevronLeftIcon, ChevronRightIcon } from "vue-tabler-icons";
import DownloadTable from "../DownloadTable.vue";
</script>

<template>
  <div class="m-2 md:m-5">
    <div class="grid grid-cols-1 md:grid-cols-4">
      <div class="md:mr-5">
        <img class="rounded shadow" src="https://placeimg.com/1080/1920/nature" alt="">
      </div>
      <div class="md:col-span-3">
        <h2 class="text-3xl mb-4">Episode I: The Phantom Menace</h2>
        <span>Do ex eiusmod commodo laboris aliqua consectetur incididunt tempor incididunt esse aliquip. Irure cupidatat et laborum cillum aute qui enim. Non eiusmod enim dolor excepteur do minim. Consectetur proident amet et pariatur eiusmod fugiat culpa minim nostrud mollit consectetur.</span>
        <DownloadTable />

        <div class="flex justify-between w-full mt-10">
          <RouterLink :to="{ name: routeDetailMovie }" class="btn btn-primary btn-disabled">
            <ChevronLeftIcon />&nbsp;None
          </RouterLink>
          <RouterLink :to="{ name: routeDetailMovie }" class="btn btn-primary">
            Episode II: The Clone Wars&nbsp;<ChevronRightIcon />
          </RouterLink>
        </div>
      </div>
    </div>
  </div>
</template>
