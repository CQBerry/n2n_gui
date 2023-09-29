<script setup lang="ts">
import { useStore } from '@/stores/properties';

const props = useStore()
</script>

<template>
    <v-btn type="submit" variant="outlined" 
    :style="`background-color: ${props.serviceStarted ? '#db7d7d' : '#93ce95'};`">
        {{ props.serviceStarted ? "Stop" : "Start" }}
    </v-btn>
</template>