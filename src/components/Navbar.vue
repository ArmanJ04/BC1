<template>
    <div class="body">
        <div class="container">
            <nav class="navbar">
                <div class="nav-left">
                    <button @click="$router.push('/')">My profile</button>
                    <button @click="$router.push('/requests')">Income requests</button>
                </div>
                <div class="nav-center">
                    <input v-model="searchAddress" type="text" placeholder="Input address" name="searchAddress" />
                    <button @click="searchUser">Search</button>
                </div>
                <div class="nav-right">
                    <button @click="connectWallet">Connect wallet</button>
                </div>
            </nav>
        </div>
    </div>
</template>

<script>
import { mapActions } from 'vuex'
export default {
    name: 'navbar',
    data() {
        return {
            searchAddress: "",
            user: {}
        };
    },
    methods: {
        ...mapActions({
            connectWallet: "connectWallet",
            getUserProfile: "getUserProfile",
        }),
        async getUser() {
            console.log(this.$store.state.address)
            this.user = await this.getUserProfile([this.$store.state.address])
        },
        async searchUser() {
            this.$router.push(`/user/${this.searchAddress}`)
            this.searchAddress = ""
        },
    },
};
</script>

<style scoped>
body {
    margin: 0;
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    background-color: #f4f4f9;
}

.navbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    background-color: #00695c;
    padding: 15px 30px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    box-sizing: border-box;
}

.nav-left, .nav-right, .nav-center {
    display: flex;
    align-items: center;
    gap: 15px;
}

.nav-center {
    flex-grow: 1;
    justify-content: center;
}

button {
    background-color: #004d40;
    border: 2px solid #004d40;
    color: #ffffff;
    font-size: 16px;
    cursor: pointer;
    padding: 10px 20px;
    border-radius: 4px;
    transition: background-color 0.3s, border-color 0.3s;
}

button:hover {
    background-color: #00796b;
    border-color: #00796b;
}

input[type="text"] {
    padding: 10px;
    font-size: 16px;
    border: 2px solid #004d40;
    border-radius: 4px;
    width: 300px;
    transition: width 0.3s, border-color 0.3s;
}

input[type="text"]:focus {
    width: 350px;
    border-color: #00796b;
}
</style>
