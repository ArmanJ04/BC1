<template>
    <div class="body">
        <div class="container">
            <div class="main-body">
                <div class="form-container">
                    <div class="back-button" @click="goBack"></div>
                    <div class="user-profile">
                        <div class="input-field">
                            <i class="fas fa-user"></i>
                            <input v-model="name" type="text" placeholder="Имя" name="first_name" required class="form-input" />
                        </div>
                        <textarea v-model="bio" placeholder="Описание" name="bio" rows="4" class="form-input"></textarea>
                        <div class="input-field">
                            <input type="file" @change="handleFileChange" accept=".jpg, .jpeg, .png" class="form-input" />
                        </div>
                    </div>
                    <div class="button-container">
                        <button @click="refresh" class="btn btn-green">Refresh </button>
                        <button @click="updateProfile" class="btn btn-blue">Edit</button>
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
            updateProfileAction: "updateProfile"
        }),
        async updateProfile() {
            try {
                const name = this.name;
                const bio = this.bio;
                let profilePictureCID = "";

                if (this.selectedFile) {
                    const ipfsHash = await this.uploadFileToPinata([this.selectedFile]);
                    profilePictureCID = ipfsHash;
                }
                console.log("A");
                await this.updateProfileAction([name, bio, profilePictureCID]);
                console.log("B");

                alert("Profile edited succesfully!");
                this.$router.push({ name: 'main' });
            } catch (error) {
                console.error("Error in updating profile:", error);
                alert("Error in updating profile:");
                this.$router.push({ name: 'main' });
            }
        },
        handleFileChange(event) {
            this.selectedFile = event.target.files[0];
        },
        async getUser() {
            console.log(this.$store.state.address);
            this.user = await this.getUserProfile([this.$store.state.address]);
            this.name = this.user.name;
            this.bio = this.user.bio;
            this.friends = this.user.friends;
            console.log(this.friends);
        },
        async refresh() {
            await this.getUser();
        },
        goBack() {
            this.$router.push('/');
        },
    },
    async mounted() {
        await this.getUser();
    },
};
</script>

<style scoped>
.body {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    background-color: #e0f7fa;
}

.container {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
}

.main-body {
    width: 100%;
    display: flex;
    justify-content: center;
}

.form-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    background-color: #ffffff;
    padding: 30px;
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.back-button {
    position: absolute;
    top: -20px;
    left: -20px;
    width: 30px;
    height: 30px;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='M19 12H5M12 19l-7-7 7-7'/%3E%3C/svg%3E");
    background-size: cover;
    cursor: pointer;
}

.user-profile {
    width: 100%;
}

.input-field {
    position: relative;
    width: 100%;
    margin-bottom: 20px;
}

.input-field i {
    position: absolute;
    top: 50%;
    left: 10px;
    transform: translateY(-50%);
    color: #00796b;
}

.form-input {
    width: calc(100% - 20px);
    padding: 10px;
    font-size: 16px;
    border: 1px solid #b0bec5;
    border-radius: 4px;
    transition: border-color 0.3s ease;
}

.form-input:focus {
    border-color: #00796b;
}

.button-container {
    display: flex;
    justify-content: space-around;
    width: 100%;
}

.btn {
    background-color: #2196f3;
    color: white;
    border: none;
    padding: 10px 20px;
    font-size: 16px;
    cursor: pointer;
    transition: background-color 0.3s ease;
    border-radius: 4px;
}

.btn:hover {
    background-color: #1976d2;
}

.btn-green {
background-color: #4caf50;
}

.btn-green
{
background-color: #388e3c;
}
</style>
