<script setup lang="ts">
const props = defineProps(['repo', 'files'])
import { invoke } from '@tauri-apps/api/tauri'


</script>

<template>
    <ul class="block w-full bg-gray-50">
        <div class="grid grid-cols-2 pb-4">
            <div class="text-center font-bold">
                <h1>Status</h1>
            </div>
            <div class="text-center font-bold">
                <h1>File Name</h1>
            </div>
        </div>
        <hr class="border-1 pt-1">
        <li v-for="item in props.files" :key="item.fileName" class="py-2">
            <div class="grid grid-cols-2">
                <div class="text-center">
                    {{ item.status }}
                </div>
                <div class="text-center">
                    {{ item.fileName }}
                </div>
            </div>
            <hr class="border-1">
        </li>
    </ul>

</template>

