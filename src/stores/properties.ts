import { defineStore } from 'pinia';

export const useStore = defineStore({
    id: 'properties',
    state: () => ({
        servers: [] as string[],
        server: "",
        virtualNetworkIP: "",
        communityName: "",
        encryptionKey: "",
        enableLogging: false,
        serviceStarted: false
    }),
    actions: {
        load() {
            if (localStorage.getItem('servers')) {
                this.servers = JSON.parse(localStorage.getItem('servers') as string)
            }
            this.server = localStorage.getItem('server') ?? ''
            this.virtualNetworkIP = localStorage.getItem('virtualNetworkIP') ?? ''
            this.communityName = localStorage.getItem('communityName') ?? ''
            this.encryptionKey = localStorage.getItem('encryptionKey') ?? ''
            this.enableLogging = (localStorage.getItem('enableLogging') === '1' ? true : false) ?? false
        },
        save() {
            localStorage.setItem('servers', JSON.stringify(this.servers))
            localStorage.setItem('server', this.server)
            localStorage.setItem('virtualNetworkIP', this.virtualNetworkIP)
            localStorage.setItem('communityName', this.communityName)
            localStorage.setItem('encryptionKey', this.encryptionKey)
            localStorage.setItem('enableLogging', this.enableLogging ? '1' : '0')
        },
        validateServer(server: string) {
            if (!server) {
                return false
            }
            if (server.includes(":") && !server.replace(':', '').includes(":")) {
                const [address] = server.split(':')

                if (address.includes(".")) {
                    return true
                }
                return false
            }
            return false
        },
        validateVirtualIP(virtualIP: string) {
            const pattern = /^((25[0-5]|2[0-4]\d|[01]?\d\d?)\.){3}(25[0-5]|2[0-4]\d|[01]?\d\d?)/g
            return pattern.test(virtualIP)
        }
    }
})