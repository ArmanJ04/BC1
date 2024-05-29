<template>
    <div class="friend-card p-3 mb-3 shadow">
        <div class="d-flex align-items-center justify-content-between">
            <div class="d-flex align-items-center" @click="searchUser">
                <img :src="user.image" alt="No Image" class="friend-avatar img-fluid rounded-circle mr-3" width="100" />
                <div class="friend-details">
                    <h3 class="friend-name">Nickname: {{ user.name }}</h3>
                </div>
            </div>
            <button @click="remove" class="btn btn-danger">Remove </button>
        </div>
    </div>
</template>

<script>
import { mapActions } from 'vuex'
export default {
    name: 'friend-item',
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
            declineFriendRequest: "declineFriendRequest",
            removeFriend: "removeFriend"
        }),
        async getUser() {
            console.log(this.userAddress)
            this.user = await this.getUserProfile([this.userAddress])
            console.log(this.user.hasMintedNft)
        },
        async searchUser() {
            this.$router.push(`/user/${this.userAddress}`)
            this.searchAddress = ""
        },
        async remove() {
            await this.removeFriend([this.userAddress])
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
.friend-card {
    background-color: #e0f7fa;
    border-radius: 8px;
    padding: 20px;
}

.friend-avatar {
    cursor: pointer;
}

.friend-details {
    cursor: pointer;
}

.friend-name {
    margin-bottom: 5px;
    color: #00796b;
}

.btn-danger {
    color: #fff;
    background-color: #0288d1;
    border-color: #0288d1;
}

.btn-danger:hover {
    background-color: #0277bd;
    border-color: #0277bd;
}

.btn-danger:focus {
    box-shadow: 0 0 0 0.2rem rgba(2, 136, 209, 0.5);
}

.btn-danger:active {
    background-color: #0277bd;
    border-color: #01579b;
}
</style>
