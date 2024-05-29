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
                <button @click="accept" class="btn btn-green">Accept Request</button>
                <button @click="decline" class="btn btn-red ml-2">Decline Request</button>
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
    background-color: #e0f7fa;
    border-radius: 10px;
    padding: 20px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.profile-picture {
    width: 80px;
    height: 80px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: #b2dfdb;
    border-radius: 50%;
}

.profile-picture img {
    max-width: 100%;
    height: auto;
    border-radius: 50%;
}

.user-details {
    margin-left: 20px;
}

.user-details h1,
.user-details h2 {
    color: #00796b;
}

.buttons-container {
    display: flex;
    gap: 10px;
}

.btn-green {
    background-color: #4caf50;
    color: #fff;
    border: none;
    padding: 10px 20px;
    font-size: 16px;
    cursor: pointer;
    border-radius: 4px;
    transition: background-color 0.3s ease;
}

.btn-green:hover {
    background-color: #388e3c;
}

.btn-red {
    background-color: #f44336;
    color: #fff;
    border: none;
    padding: 10px 20px;
    font-size: 16px;
    cursor: pointer;
    border-radius: 4px;
    transition: background-color 0.3s ease;
}

.btn-red:hover {
    background-color: #d32f2f;
}
</style>
