<template>
    <div class="body">
        <div class="container">
            <div class="main-body">
                <div class="row">
                    <div class="col-lg-8">
                        <div class="user-profile">
                            <div class="profile-header">
                                <div class="profile-image">
                                    <img :src="userImageSrc" alt="No Image" class="avatar" />
                                </div>
                                <div class="profile-info">
                                    <h4 class="nickname">{{ user.name }}</h4>
                                    <div v-if="hasNFT">
                                        <h5 class="nft-label">TOPWEB3 NFT</h5>
                                        <a href="https://sepolia.etherscan.io/address/0x8dE238A81042E99FFCb25666f281429D1EAf59F6">
                                            <img class="nft-image" src="../image/nft.jpg" alt="NFT Image">
                                        </a>
                                    </div>
                                </div>
                            </div>
                            <div class="profile-actions">
                                <button @click="refresh" class="btn">Обновить входящие данные</button>
                                <button @click="goToEditPage" class="btn">Редактировать профиль</button>
                            </div>
                            <div class="user-bio">
                                <h4>Описание: {{ user.bio || 'Нет описания' }}</h4>
                            </div>
                            <div class="profile-details">
                                <h5>Адрес: {{ $store.state.address }}</h5>
                                <div>
                                    <div v-if="friends.length >= 2 && !hasNFT" >
                                        <button @click="getNFT" class="btn">Get NFT</button>
                                    </div>
                                </div>
                            </div>
                            <div class="create-post" v-if="friends.length >= 2">
                                <h4>Написать пост</h4>
                                <textarea v-model="postContent" class="form-control" rows="3" placeholder="Что у вас нового?"></textarea>
                                <button @click="createPost" class="btn">Отправить</button>
                            </div>
                            <div class="user-posts" v-if="friends.length >= 2">
                                <h4>Ваши посты</h4>
                                <div v-if="posts.length === 0">
                                    <p>Нет постов для отображения</p>
                                </div>
                                <div v-else>
                                    <div v-for="post in posts" :key="post.id" class="post-item">
                                        <h1>{{ post.id }}</h1>
                                        <p>{{ post.content }}</p>
                                        <!-- <small>{{ post.timestamp | formatDate }}</small> -->
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="col-lg-4">
                        <div class="user-friends">
                            <h1>Друзья</h1>
                            <div v-for="address in friends" :key="address">
                                <friend-item :userAddress="address" />
                            </div>
                        </div>
                    </div>
                </div>
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
            friends: [],
            showNameForm: false,
            showBioForm: false,
            showImageForm: false,
            hasNFT: false,
            postContent: "",
            posts: []
        };
    },
    computed: {
        userImageSrc() {
            return this.user.image ? this.user.image : 'https://upload.wikimedia.org/wikipedia/commons/thumb/2/2c/Default_pfp.svg/1200px-Default_pfp.svg.png';
        },
    },
    methods: {
        ...mapActions({
            connectWallet: "connectWallet",
            getUserProfile: "getUserProfile",
            updateProfileName: "updateProfileName",
            updateProfileBio: "updateProfileBio",
            updateProfilePicture: "updateProfilePicture",
            uploadFileToPinata: "uploadFileToPinata",
            getImageFromPinata: "getImageFromPinata",
            getTOPWEB3: "getTOPWEB3",
            getBalanceNFT: "getBalanceNFT",
            setNftAction: "setNft",
            createPostAction: "createPost",
            getUserPostsAction: "getUserPosts"
        }),
        async getUser() {
            console.log(this.$store.state.address)
            this.user = await this.getUserProfile([this.$store.state.address])
            this.friends = this.user.friends
            this.hasNFT = this.user.hasMintedNft
            console.log(this.friends)
            console.log(this.user.hasMintedNft)
        },
        async getNFT() {
            await this.getTOPWEB3([this.$store.state.address, this.friends.length])
            await this.setNftAction([this.$store.state.address, true])
            this.hasNFT = true
        },
        async getBalance() {
            this.hasNFT = await this.getBalanceNFT([this.$store.state.address])
            console.log(this.hasNFT)
        },
        async getUserPosts(){
            this.posts = await this.getUserPostsAction([this.$store.state.address])
        },
        async refresh() {
            await this.getUser()
            await this.getBalance()
            await this.getUserPosts()
        },
        goToEditPage() {
            this.$router.push({ name: 'edit' });
        },
        async createPost(){
            await this.createPostAction([this.postContent])
            this.postContent = ""
        },
    },
    async mounted() {
        await this.refresh()
    },
    // watch: {
    //     'this.hasNFT': 'getBalance'
    // }
};
//алия лучшая 
</script>
<style scoped>
body {
    margin: 0;
    font-family: Arial, sans-serif;

}

.container {
    display: flex;
    justify-content: center;
    padding: 20px;
}

.main-body {
    width: 100%;
    display: flex;
}

.row {
    display: flex;
    justify-content: space-between;
    width: 100%;
}

.col-lg-8, .col-lg-4 {
    padding: 10px;
}

.user-profile {
    background-color: #f8f9fa;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
}

.profile-header {
    display: flex;
    align-items: center;
}

.profile-image .avatar {
    width: 100px;
    height: 100px;
    border-radius: 50%;
    object-fit: cover;
    margin-right: 20px;
}

.profile-info {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
}

.nickname {
    margin: 0;
}

.nft-label {
    margin: 5px 0 0;
}

.nft-image {
    width: 50px;
    height: 50px;
    border-radius: 50%;
    border: 2px solid black;
}

.profile-actions {
    margin-top: 20px;
    display: flex;
    gap: 10px;
}

.user-bio, .profile-details, .create-post, .user-posts {
    margin-top: 20px;
}

textarea.form-control {
    width: 90%;
    padding: 10px;
    font-size: 16px;
    border: 1px solid #ccc;
    border-radius: 4px;
    resize: none;
}

button.btn {
    background-color: #007bff;
    color: white;
    border: none;
    padding: 10px 20px;
    font-size: 16px;
    cursor: pointer;
    transition: background-color 0.3s ease;
}

button.btn:hover {
    background-color: #0056b3;
}

.user-friends {
    background-color: #f8f9fa;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
    text-align: center;
}

.user-friends h1 {
    margin-bottom: 20px;
}

.post-item {
    padding: 10px;
    border-bottom: 1px solid #ccc;
    margin-bottom: 10px;
}
</style>
