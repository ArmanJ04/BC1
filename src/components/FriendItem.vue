<template>
    <div class="friend-card p-3 mb-3 shadow">
        <div class="d-flex align-items-center">
            <img @click="searchUser" :src="user.image" alt="No Image" class="friend-avatar img-fluid rounded-circle mr-3" width="100" />
            <div @click="searchUser" class="friend-details">
                <h3 class="friend-name">Никнейм: {{ user.name }}</h3>
            </div>
        </div>
        <button @click="remove" class="btn btn-danger ml-auto">Удалить из друзей</button>
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
    background-color: #fff;
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
}

.btn-danger {
    color: #fff;
    background-color: #dc3545;
    border-color: #dc3545;
}

.btn-danger:hover {
    background-color: #c82333;
    border-color: #bd2130;
}

.btn-danger:focus {
    box-shadow: 0 0 0 0.2rem rgba(220, 53, 69, 0.5);
}

.btn-danger:active {
    background-color: #bd2130;
    border-color: #b21f2d;
}
</style>