<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
const popup = defineModel('popup')
const dir = defineModel('dir')
const props = defineProps(['repo'])
const emit =defineEmits(['updateFiles'])
async function submitDir(e:any){
    e.preventDefault()
    popup.value = false
    await invoke("makedir", {newrepo: dir.value, name: props.repo}) 
    emit('updateFiles')
}

</script>

<template>
    <div class="relative z-10">
        <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"></div>
        <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
            <div class="flex min-h-full items-center justify-center p-4 text-center">
                <form v-on:submit="submitDir">
                    <div class="flex text-center bg-gray-200">
                        <label for="repo" class="block font-bold text-sm text-center">Enter Directory Name</label>
                        <div class="flex mt-1 my-2 px-2 justify-center">
                            <input id="repo" name="repo" type="text" v-model="dir" required class="block w-full bg-gray-100 rounded-md pl-1 m-1 shadow-md">
                        </div>
                    </div>
                </form>
            </div>
        </div>

    </div>


</template>