<script setup lang="ts">
    import { ref } from 'vue';
    import { invoke } from '@tauri-apps/api/tauri'
    const props = defineProps(['selected', 'repo'])
    const emit =defineEmits(['updateFiles'])
    const msg = ref<string>("")

    async function svnAdd() {
        await invoke("add", {filelist: props.selected, name: props.repo}) //void so wtf am I doing
        emit('updateFiles')
    }
    async function svnDelete() {
        await invoke("delete", {filelist: props.selected, name: props.repo})
        emit('updateFiles')
    }
    async function svnCommit() {
        console.log(msg.value, props.repo)
        await invoke("commit", {message: msg.value, name: props.repo})
        emit('updateFiles')
    }
    async function svnRevert() {
        await invoke("revert", {filelist: props.selected, name: props.repo})
        emit('updateFiles')
    }
</script>

<template>
    <div class="grid grid-cols-5 h-full w-full bg-slate-600">
        <div class="grid grid-cols-1 justify-items-stretch h-full w-full">
            <input id="repo" name="repo" v-model = "msg" type="text" placeholder="commit message" required class="block w-full bg-gray-100 rounded-md my-4 px-1 ring-1 ring-gray-400 shadow-md {{ if active == item ? 'text-blue-100' :'text-red-400'}}"></input>
        </div>
        <div class="grid grid-cols-1 justify-items-stretch h-full w-full">
            <button 
                @click="svnAdd"
                class=" m-3 py-2.5 px-2.5 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-100 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700"
            >Add</button>
        </div>
        <div class="grid grid-cols-1 justify-items-stretch h-full w-full">
            <button 
                @click="svnCommit"
                class="py-2.5 px-5 m-3 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-100 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700"
            >Commit</button>
        </div>
        <div class="grid grid-cols-1 justify-items-stretch h-full w-full">
            <button
                @click="svnDelete"
                class="py-2.5 px-5 m-3 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-100 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700"
            >Delete</button>
        </div>
        <div class="grid grid-cols-1 justify-items-stretch h-full w-full">
            <button
                @click="svnRevert"
                class="py-2.5 px-5 m-3 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-100 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700"
            >Revert</button>
        </div>
    </div>
</template>