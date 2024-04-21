<script setup lang="ts">
    import { ref } from 'vue';
    import { invoke } from '@tauri-apps/api/tauri'
    const filesToAdd = ref(['']);
    const toAddInput = ref({
        filelist: "",
        name: "",
    })
    const commit = ref('')
    const msg = ref('')

    async function svnAdd(toAdd: string[]) {
        filesToAdd.value = await invoke("add", {filelist: toAdd, name: toAddInput.value.name}) //void so wtf am I doing
    }
    async function svnCommit(message: string) {
        commit.value = await invoke("commit", {message: message})
    }
</script>

<template>
    <div class="grid grid-cols-4 h-[20%] w-screen bg-slate-600">
        <div class="flex justify-center items-center">
            <input id="repo" name="repo" v-model = "msg" type="text" required class="block w-full bg-gray-100 rounded-md py-1 pl-2 mt-2 ring-1 ring-gray-400 shadow-md {{ if active == item ? 'text-blue-100' :'text-red-400'}}"></input>
        </div>
        <div class="flex justify-center items-center">
            <button 
                @click="svnAdd(['test', 'tets'])"
                class="py-2.5 px-5 me-2 mb-2 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-100 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700"
            >Add</button>
        </div>
        <div class="flex justify-center items-center">
            <button 
                @click="svnCommit(msg)"
                class="py-2.5 px-5 me-2 mb-2 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-100 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700"
            >Commit</button>
        </div>
        <div class="flex justify-center items-center">
            <p class="border border-solid text-white">Maybe a Status thing?</p>
        </div>
    </div>
</template>