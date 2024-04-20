<script setup="ts">

import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const temporaryFiles = ref(['helloWorld.cpp', "helloWorld.rs", "helloWorld.css", "helloWorld.vue"])

</script>

<template>
    <div class="grid">

    </div>
</template>