<script setup lang="ts">
const props = defineProps(['repo'])
</script>

<template>
    <div class="flex">{{ props.repo }}</div>

</template>
