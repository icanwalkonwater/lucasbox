<script setup lang="ts">
import { HomeIcon, MovieIcon, UploadIcon, SettingsIcon } from 'vue-tabler-icons';
</script>

<template>
    <div class="drawer drawer-mobile">
        <input id="main-drawer" type="checkbox" class="drawer-toggle" />
        <div class="drawer-content">

            <div class="navbar border-b border-base-300">
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
            </div>

        </div>
        <div class="drawer-side border-r border-base-300">
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
    </div>
</template>