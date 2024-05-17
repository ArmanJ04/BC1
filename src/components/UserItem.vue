<template>
    <div class="user-container">
      <div class="user-image">
        <img :src="user.image" alt="User Image" class="user-avatar" />
      </div>
      <div class="user-details">
        <h2>{{ user.name }}</h2>
        <p><strong>Адрес:</strong> {{ user.address }}</p>
        <p><strong>Описание:</strong> {{ user.bio || 'Описания пока нет' }}</p>
      </div>
      <div class="user-actions">
        <button @click="getUser" class="btn btn-blue">Обновить</button>
        <div v-if="isFriend">
          <button @click="sendRequest" :disabled="buttonDisabled" class="btn btn-blue">{{ buttonText }}</button>
        </div>
        <div v-else>
          <p class="text-success">Уже в друзьях</p>
        </div>

        <div class="user-posts">
                                <h4>Посты пользователя</h4>
                                <div v-if="posts.length === 0">
                                    <p>Нет постов для отображения</p>
                                </div>
                                <div v-else>
                                    <div v-for="post in posts" :key="post.id" class="post-item">
                                        <h1>{{ post.id }}</h1>
                                        <p>{{ post.content }}</p>
                                        <hr>
                                        <!-- <small>{{ post.timestamp | formatDate }}</small> -->
                                    </div>
                                </div>
                            </div>
      </div>
    </div>
  </template>


<script>
import { mapActions } from 'vuex'
export default {
    name: "user-item",
    data() {
        return {
            user: {},
            hasWatched: false,
            buttonText: "Отправить запрос",
            isFriend: false,
            buttonDisabled: false,
            posts: []
        }
    },
    props: {
        userAddress: {
            type: String,
            require: true
        }
    },
    methods: {
        ...mapActions({
            getUserProfile: 'getUserProfile',
            connectWallet: "connectWallet",
            sendFriendRequest: "sendFriendRequest",
            getUserPostsAction: "getUserPosts"
        }),
        async getUser() {
            console.log(this.userAddress)
            this.user = await this.getUserProfile([this.userAddress])
            console.log(this.user.friends.includes(this.$store.state.address))
            if (!this.user.friends.includes(this.$store.state.address)) {
                this.isFriend = true
            }
            else {
                this.isFriend = false
            }
            console.log(this.user)
        },
        async sendRequest() {
            await this.sendFriendRequest(this.userAddress)
            this.buttonText = "Запрос отправлен"
            this.buttonDisabled = true
        },
        async getUserPosts(){
            this.posts = await this.getUserPostsAction([this.userAddress])
        },
        async refresh() {
            await this.getUser()
            await this.getUserPosts()
        },
    },
    watch: {
        '$route.params.searchAddress': 'getUser'
    },
    async mounted() {
        await this.refresh()
    }
}
</script>

<style scoped>
.user-container {
  background-color: #f8f9fa;
  border-radius: 8px;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
  padding: 20px;
  max-width: 400px;
  margin: 20px auto;
}

.user-avatar {
  width: 150px;
  height: 200px;
  border-radius: 50%;
  object-fit: cover;
}

.user-details h2 {
  color: #343a40;
  margin-bottom: 10px;
}

.user-details p {
  color: #6c757d;
  margin-bottom: 5px;
}

.user-actions {
  margin-top: 20px;
}

.user-actions .btn {
  background-color: #007bff;
  color: #fff;
  border: none;
  margin-right: 10px;
}

.user-actions .btn:hover {
  background-color: #0056b3;
}
</style>

