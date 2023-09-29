<script setup lang="ts">
import { useStore } from '@/stores/properties';

const props = useStore()
</script>

<template>
    <v-switch label="Enable logging" color="success" v-model="props.enableLogging"></v-switch>
</template>