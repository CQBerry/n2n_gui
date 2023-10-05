<script setup lang="ts">
import { useStore } from '@/stores/properties'

const props = useStore()

const IPRules = [
    () => props.validateVirtualIP(props.virtualNetworkIP) || `The Virtual Network IP ${props.virtualNetworkIP} is illegal, it should match '10.x.x.x', '172.16.0.0-172.31.255.254' or '192.168.x.x' or just leave it empty`
]

</script>

<template>
    <v-text-field v-model="props.virtualNetworkIP" label="Virtual Network IP" variant="outlined"
        :rules="IPRules"></v-text-field>
</template>