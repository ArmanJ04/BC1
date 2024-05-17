<template>
    <div class="container mt-4">
        <h2 class="title mb-3">Pending Friend Requests</h2>
        <button @click="getRequests" class="btn btn-green mb-4">Refresh</button>
        <div class="requests-list">
            <div v-for="address in requests" :key="address" class="request-item">
                <request-item :userAddress="address" />
            </div>
        </div>
    </div>
</template>

<script>
import { mapActions } from 'vuex'
export default {
    name: 'MainView',
    data() {
        return {
            name: "",
            bio: "",
            selectedFile: null,
            user: {},
            requests: []
        };
    },
    methods: {
        ...mapActions({
            getincomingFriendRequests: "getincomingFriendRequests"
        }),
        async getRequests() {
            console.log(this.$store.state.address)
            this.requests = await this.getincomingFriendRequests([this.$store.state.address])
            console.log(this.requests)
        }
    },
    async mounted() {
        this.getRequests()
    },
    watch: {
        'this.requests': 'getRequests'
    }
};
//алия лучшая
</script>

<style scoped>
.container {
    background-color: #f8f9fa;
    border-radius: 8px;
    padding: 20px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}
.title {
    font-size: 24px;
    font-weight: bold;
    text-align: center;
    color: #333;
}
.btn-green {
    background-color: #28a745;
    color: #fff;
    border: none;
    padding: 10px 20px;
    border-radius: 5px;
    font-size: 16px;
    cursor: pointer;
    transition: background-color 0.3s ease;
    margin-bottom: 10px;
}
.btn-green:hover {
    background-color: #218838;
}
.requests-list {
    display: flex;
    flex-direction: column;
    gap: 15px;
}
.request-item {
    background-color: #ffffff;
    border-radius: 8px;
    padding: 15px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    transition: transform 0.3s ease, box-shadow 0.3s ease;
}
.request-item:hover {
    transform: translateY(-5px);
    box-shadow: 0 6px 12px rgba(0, 0, 0, 0.1);
}
</style>