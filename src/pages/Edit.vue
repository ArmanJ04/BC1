<template>
    <div class="body">
        <div class="container">
            <div class="main-body">
                <div class="user-profile">
                    <div class="form-container">
                        <div class="back-button" @click="goBack"></div>
                        <div class="input-field">
                            <i class="fas fa-file"></i>
                            <input v-model="name" type="text" placeholder="Имя" name="first_name" required class="form-input" />
                        </div>
                        <textarea v-model="bio" placeholder="Описание" name="bio" rows="4" class="form-input"></textarea>
                        <div class="input-field">
                            <input type="file" @change="handleFileChange" accept=".jpg, .jpeg, .png" class="form-input" />
                        </div>
                        <button @click="refresh" class="btn btn-blue mt-3">Обновить входящие данные</button>
                        <button @click="updateProfile" class="btn btn-blue">Изменить данные аккаунта</button>
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
            // try {
            //     if (this.name !== this.user.name) {
            //         await this.updateProfileName([this.name]);
            //     }
            //     if (this.bio !== this.user.bio) {
            //         await this.updateProfileBio([this.bio]);
            //     }
            //     if (this.selectedFile) {
            //         console.log(this.selectedFile)
            //         const ipfsHash = await this.uploadFileToPinata([this.selectedFile])
            //         await this.updateProfilePicture([ipfsHash])
            //     }
            //     alert("Профиль обновлен успешно!");
            //     this.$router.push({ name: 'main' });
            // } catch (error) {
            //     console.error("Ошибка при обновлении профиля:", error);
            //     alert("Произошла ошибка при обновлении профиля.");
            //     this.$router.push({ name: 'main' });
            // }
            try {
                const name = this.name
                const bio = this.bio 
                let profilePictureCID = "";

                if (this.selectedFile) {
                    const ipfsHash = await this.uploadFileToPinata([this.selectedFile]);
                    profilePictureCID = ipfsHash;
                }
                console.log("A")
                await this.updateProfileAction([ name, bio, profilePictureCID ]);
                console.log("B")

                alert("Профиль обновлен успешно!");
                this.$router.push({ name: 'main' });
            } catch (error) {
                console.error("Ошибка при обновлении профиля:", error);
                alert("Произошла ошибка при обновлении профиля.");
                this.$router.push({ name: 'main' });
            }
        },
        handleFileChange(event) {
            this.selectedFile = event.target.files[0];
        },
        async getUser() {
            console.log(this.$store.state.address)
            this.user = await this.getUserProfile([this.$store.state.address])
            this.name = this.user.name;
            this.bio = this.user.bio;
            this.friends = this.user.friends
            console.log(this.friends)
        },
        async refresh() {
            await this.getUser()
        },
        goBack() {
            this.$router.push('/');
        },
    },
    // async mounted() {
    //     this.getBalance()
    // },
    // watch: {
    //     'this.hasNFT': 'getBalance'
    // }
    async mounted() {
        await this.getUser();
    },
};
//алия лучшая 
</script>
<style scoped>

.body {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
}

.container {
    display: flex;
    justify-content: center;
    align-items: center;
}

.main-body {
    width: 100%;
    display: flex;
    justify-content: center;
}

.user-profile {
    background-color: #f8f9fa;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
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


.form-container {
    position: relative;
    width: 400px; /* Ширина формы */
    margin: 0 auto; /* Центрирование формы */
    text-align: center; /* Центрирование дочерних элементов */
}

.form-input {
    width: calc(100% - 20px); /* Ширина поля с учетом отступов */
    margin: 10px 0; /* Отступ между элементами формы */
    padding: 10px;
    font-size: 16px;
    border: 1px solid #ccc;
    border-radius: 4px;
    resize: none;
}

.input-field {
    position: relative;
    margin: 10px 0; /* Отступ между элементами формы */
}

.input-field i {
    position: absolute;
    top: 50%;
    left: 10px;
    transform: translateY(-50%);
}

.btn {
    background-color: #007bff;
    color: white;
    border: none;
    padding: 10px 20px;
    font-size: 16px;
    cursor: pointer;
    transition: background-color 0.3s ease;
    margin-top: 5px;
    margin-bottom: 5px
}

.btn:hover {
    background-color: #0056b3;
}

</style>


