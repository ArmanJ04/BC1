<template>
    <div class="body">
        <div class="container">
            <nav class="navbar">
                <div class="btn-container">
                    <button @click="$router.push('/')">Мой профиль</button>
                    <button @click="$router.push('/requests')">Входящие запросы</button>
                </div>
                <div class="nav-buttons">
                    <input v-model="searchAddress" type="text" placeholder="Введите адрес" name="searchAddress" />
                    <button @click="searchUser">Поиск</button>
                </div>
                <div class="btn-container">
                    <button @click="connectWallet">Подключить кошелек</button>
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
            name: "",
            bio: "",
            selectedFile: null,
            searchAddress: "",
            user: {}
        };
    },
    methods: {
        ...mapActions({
            connectWallet: "connectWallet",
            getUserProfile: "getUserProfile",
            // changeNetwork : "changeNetwork"
        }),
        handleFileChange(event) {
            this.selectedFile = event.target.files[0];
        },
        async getUser() {
            console.log(this.$store.state.address)
            this.user = await this.getUserProfile([this.$store.state.address])
        },
        async searchUser() {
            this.$router.push(`/user/${this.searchAddress}`)
            this.searchAddress = ""
        },
        // async change(){
        //     await this.changeNetwork([1])
        // }
    },
};
</script>

<style scoped>
body {
    margin: 0;
    font-family: Arial, sans-serif;
}

.navbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    background-color: black;
    padding: 10px 20px;
    box-sizing: border-box;
}

.btn-container, .nav-buttons {
    display: flex;
    gap: 10px;
}

button {
    background: none;
    border: none;
    color: white;
    font-size: 16px;
    cursor: pointer;
    padding: 10px;
    transition: color 0.3s ease;
}

button:hover {
    color: rgba(255, 255, 255, 0.8);
}

input[type="text"] {
    padding: 10px;
    font-size: 16px;
    border: 1px solid #ccc;
    border-radius: 4px;
    width: 300px; /* увеличиваем ширину поля ввода */
    transition: width 0.3s ease;
}

input[type="text"]:focus {
    width: 350px; /* немного увеличиваем ширину при фокусе */
}
</style>
