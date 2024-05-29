<template>
  <div class="user-container">
    <div class="user-image">
      <img :src="user.image" alt="User Image" class="user-avatar" />
    </div>
    <div class="user-details">
      <h2>{{ user.name }}</h2>
      <p><strong>Address:</strong> {{ user.address }}</p>
      <p><strong>Description:</strong> {{ user.bio || 'No description' }}</p>
    </div>
    <div class="user-actions">
      <button @click="refresh" class="btn btn-green">Refresh</button>
      <div v-if="isFriend">
        <button @click="sendRequest" :disabled="buttonDisabled" class="btn btn-green">{{ buttonText }}</button>
      </div>
      <div v-else>
        <p class="text-success">Already friends</p>
      </div>
    </div>
    <div class="user-posts">
      <h4>User's post</h4>
      <div v-if="posts.length === 0">
        <p>No post</p>
      </div>
      <div v-else>
        <div v-for="post in posts" :key="post.id" class="post-item">
          <h1>{{ post.id }}</h1>
          <p>{{ post.content }}</p>
          <hr />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { mapActions } from 'vuex'
export default {
  name: 'user-item',
  data() {
    return {
      user: {},
      buttonText: 'Отправить запрос',
      isFriend: false,
      buttonDisabled: false,
      posts: []
    }
  },
  props: {
    userAddress: {
      type: String,
      required: true
    }
  },
  methods: {
    ...mapActions({
      getUserProfile: 'getUserProfile',
      sendFriendRequest: 'sendFriendRequest',
      getUserPostsAction: 'getUserPosts'
    }),
    async getUser() {
      this.user = await this.getUserProfile([this.userAddress])
      this.isFriend = !this.user.friends.includes(this.$store.state.address)
    },
    async sendRequest() {
      await this.sendFriendRequest(this.userAddress)
      this.buttonText = 'Запрос отправлен'
      this.buttonDisabled = true
    },
    async getUserPosts() {
      this.posts = await this.getUserPostsAction([this.userAddress])
    },
    async refresh() {
      await this.getUser()
      await this.getUserPosts()
    }
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
  background-color: #e0f2f1;
  border-radius: 10px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  padding: 20px;
  max-width: 500px;
  margin: 20px auto;
}

.user-avatar {
  width: 150px;
  height: 150px;
  border-radius: 50%;
  object-fit: cover;
}

.user-details h2 {
  color: #004d40;
  margin-bottom: 10px;
}

.user-details p {
  color: #004d40;
  margin-bottom: 5px;
}

.user-actions {
  margin-top: 20px;
}

.user-actions .btn {
  background-color: #4caf50;
  color: #fff;
  border: none;
  padding: 10px 20px;
  font-size: 16px;
  border-radius: 5px;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.user-actions .btn:hover {
  background-color: #388e3c;
}

.user-posts {
  margin-top: 20px;
}

.post-item {
  background-color: #f1f8e9;
  padding: 10px;
  border-radius: 5px;
  margin-bottom: 10px;
}

.text-success {
  color: #4caf50;
}

.btn-red {
  background-color: #f44336;
  color: #fff;
  border: none;
  padding: 10px 20px;
  font-size: 16px;
  border-radius: 5px;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.btn-red:hover {
  background-color: #d32f2f;
}
</style>
