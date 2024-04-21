<script setup lang="ts">
//import { RefSymbol } from '@vue/reactivity';
import { ref } from 'vue';
import SubmitBar from "./SubmitBar.vue";
const props = defineProps(['repo', 'files'])
const emit = defineEmits(['updateFiles'])
const selected =ref<Array<string>>([])
function clickItem(fileName:string){
    let found=false
    selected.value = selected.value.filter((item)=>{
        if(item == fileName){
            found=true
            console.log("found")
            return false
        }
        return true
    })
    
    if(found == false)
        selected.value.push(fileName)
    console.log(selected.value)
    
    
}
function setItemCSS(item:string){
  let isActive = ""
  if (selected.value.find((e)=>e==item))
      isActive = "bg-gray-300"

  return `grid grid-cols-2 py-2 ${isActive}`
}
function updateFiles(){
    console.log("UPDATING FILES")
    emit('updateFiles')
}
</script>

<template>
    <div class="flex flex-col w-full min-h-screen justify-between">
        <ul class="block w-full h-full bg-gray-50">
            <div class="grid grid-cols-2 pb-4">
                <div class="text-center font-bold">
                    <h1>Status</h1>
                </div>
                <div class="text-center font-bold">
                    <h1>File Name</h1>
                </div>
            </div>
            <hr class="border-1 pt-1">
            <div></div>
            <li v-for="item in props.files" :key="item.fileName" class="overflow-y-auto" @click="clickItem(item.fileName)" >
                <div :class="setItemCSS(item.fileName)">
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
        <div class="w-full h-[10%]">
            <SubmitBar :selected="selected" :repo="props.repo" @update-files="updateFiles"></SubmitBar>
        </div>
    </div>
</template>

