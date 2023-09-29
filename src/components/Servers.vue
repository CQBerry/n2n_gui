<script setup lang="ts">
import { useStore } from '@/stores/properties'

const props = useStore()

const serverRules = [
    () => props.server !== "" || "Server should not be empty",
    () => props.validateServer(props.server) || `The server ${props.server} is illegal`,
    () => !props.server.includes(" ") || "Server should not contain space"
]

function handleNullValue(){
    if (props.server === null){
        props.server = ""
    }
}

</script>

<template>
    <v-combobox :onblur="handleNullValue" v-model="props.server" label="Server" variant="outlined" :items="props.servers"
        :rules="serverRules"></v-combobox>
</template>