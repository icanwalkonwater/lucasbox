<script setup lang="ts">
import PageLayout from "@/components/layout/PageLayout.vue";
import TagList from "@/components/TagList.vue";

const imgSrc = import.meta.env.VITE_DEFAULT_COVER;
</script>

<template>
  <PageLayout has-header>
    <template #header>
      <div class="flex">
        <div class="hidden md:block mr-4">
          <img class="rounded-xl aspect-poster object-cover w-80" :src="imgSrc">
        </div>
        <div>
          <h1 class="text-3xl">Titre</h1>
          <TagList :tags="[{label: 'Uwu'}]" class="my-2" />
          <p>Deserunt ullamco consectetur occaecat deserunt aliquip cillum. Nostrud commodo veniam nulla deserunt aliqua minim. Nulla occaecat nostrud cillum nisi velit occaecat aliqua do amet exercitation eiusmod sunt consectetur.</p>
        </div>
      </div>
    </template>
    <template #default>
      <div class="grid grid-cols-1 md:grid-cols-5 gap-4">
        <div
          v-for="i in 10"
          :key="i"
          class="card image-full object-contain"
        >
          <figure><img :src="imgSrc" class="aspect-video"></figure>
          <div class="card-body">
            <h2 class="card-title">Title</h2>
            <div class="flex flex-wrap">
              <TagList :tags="[{label: 'Uwu'},{label: 'Owo'}]"/>
            </div>
            <p class="text-ellipsis">Occaecat qui nostrud officia in deserunt sit dolore in non laborum cillum exercitation qui.</p>
          </div>
        </div>
      </div>
    </template>
  </PageLayout>
</template>
