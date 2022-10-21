<script setup lang="ts">
import { HomeIcon } from 'vue-tabler-icons';
</script>

<template>
    <div class="drawer drawer-mobile">
        <input id="main-drawer" type="checkbox" class="drawer-toggle" />
        <div class="drawer-content">
                
            <div class="navbar border-b border-neutral-focus">
                <div class="navbar-start">
                    <a class="btn normal-case text-xl">LucasBox</a>
                </div>
            </div>
                
        </div>
        <div class="drawer-side border-r border-neutral-focus">
            <div class="w-20">
                <img src="/favicon.ico" />
            </div>
            <ul class="menu">
                <li><a class="place-content-around"><HomeIcon /></a></li>
            </ul>
        </div>
    </div>
</template>