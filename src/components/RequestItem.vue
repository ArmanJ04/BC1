<template>
    <div class="request-item-container p-3 mb-3 shadow">
        <div class="profile-picture">
            <img v-if="user.image" :src="user.image" alt="User Image" class="img-fluid rounded-circle" />
            <div v-else>
                <p>No profile picture yet</p>
            </div>
        </div>
        <div class="user-details ml-3">
            <h1>Address: {{ userAddress }}</h1>
            <h2>Username: {{ user.name }}</h2>
            <div class="buttons-container mt-3">
                <button @click="accept" class="btn btn-success">Accept Request</button>
                <button @click="decline" class="btn btn-danger ml-2">Decline Request</button>
            </div>
        </div>
    </div>
</template>

<script>
import { mapActions } from 'vuex'
export default {
    name: 'request-item',
    data() {
        return {
            user: {}
        };
    },
    props: {
        userAddress: {
            type: String,
            required: true,
        },
    },
    methods: {
        ...mapActions({
            getUserProfile: "getUserProfile",
            acceptFriendRequest: "acceptFriendRequest",
            declineFriendRequest: "declineFriendRequest"
        }),
        async getUser() {
            console.log(this.userAddress)
            this.user = await this.getUserProfile([this.userAddress])
            console.log(this.user)
        },
        async accept() {
            await this.acceptFriendRequest([this.userAddress])
        },
        async decline() {
            await this.declineFriendRequest([this.userAddress])
        }
    },
    async mounted() {
        this.getUser()
    },
    watch: {
        'this.userAddress': 'getUser'
    }
}
</script>

<style scoped>
.request-item-container {
    display: flex;
    align-items: center;
    background-color: #fff;
    border-radius: 10px;
}

.profile-picture {
    width: 80px;
    height: 80px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: #f0f0f0;
    border-radius: 50%;
}

.profile-picture img {
    max-width: 100%;
    height: auto;
    border-radius: 50%;
}

.user-details h1,
.user-details h2 {
    color: #333;
}

.buttons-container {
    display: flex;
}

.btn-success {
    background-color: #28a745;
    border-color: #28a745;
}

.btn-danger {
    background-color: #dc3545;
    border-color: #dc3545;
}
</style>