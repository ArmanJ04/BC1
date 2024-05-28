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
                      <h5 class="nft-label">Blockchain Badge</h5>
                      <a href="https://sepolia.etherscan.io/address/0x8dE238A81042E99FFCb25666f281429D1EAf59F6">
                        <img class="nft-image" src="../image/nft.jpg" alt="NFT Image">
                      </a>
                    </div>
                  </div>
                </div>
                <div class="profile-actions">
                  <button @click="refresh" class="btn btn-blue">Update Profile Data</button>
                  <button @click="goToEditPage" class="btn btn-blue">Edit Profile</button>
                </div>
                <div class="user-bio">
                  <h4>Description: {{ user.bio || 'No description' }}</h4>
                </div>
                <div class="profile-details">
                  <h5>Address: {{ $store.state.address }}</h5>
                  <div>
                    <div v-if="friends.length >= 1 && !hasNFT" >
                      <button @click="getNFT" class="btn btn-green">Get Blockchain Badge</button>
                    </div>
                  </div>
                </div>
                <div class="create-post" v-if="friends.length >= 1">
                  <h4>Write a Post</h4>
                  <textarea v-model="postContent" class="form-control" rows="3" placeholder="What's new?"></textarea>
                  <button @click="createPost" class="btn btn-green">Send</button>
                </div>
                <div class="user-posts" v-if="friends.length >= 1">
                  <h4>Your Posts</h4>
                  <div v-if="posts.length === 0">
                    <p>No posts to display</p>
                  </div>
                  <div v-else>
                    <div v-for="post in posts" :key="post.id" class="post-item">
                      <h1>{{ post.id }}</h1>
                      <p>{{ post.content }}</p>
                      <small>{{ post.timestamp | formatDate }}</small>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <div class="col-lg-4">
              <div class="user-friends">
                <h1>Friends</h1>
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
        getBlockchainBadge: "getBlockchainBadge",
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
        await this.getBlockchainBadge([this.$store.state.address, this.friends.length])
        await this.setNftAction([this.$store.state.address, true])
        this.hasNFT = true;
        this.user.hasMintedNft = true;
    },
    async refresh() {
      await this.getUser();
    },
    async goToEditPage() {
      this.$router.push('/edit');
    },
    async createPost() {
      try {
        await this.createPostAction([this.$store.state.address, this.postContent]);
        await this.getUserPosts();
        this.postContent = '';
      } catch (error) {
        console.error("Error creating post:", error);
      }
    },
    async getUserPosts() {
      try {
        this.posts = await this.getUserPostsAction(this.$store.state.address);
      } catch (error) {
        console.error("Error fetching user posts:", error);
      }
    }
  },
  async mounted() {
    await this.getUser();
    await this.getUserPosts();
  },
  filters: {
    formatDate(value) {
      const date = new Date(value);
      return date.toLocaleString();
    }
  }
};
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

.profile-header {
  display: flex;
  align-items: center;
}

.profile-image {
  margin-right: 20px;
}

.avatar {
  width: 100px;
  height: 100px;
  border-radius: 50%;
}

.profile-info {
  flex: 1;
}

.nickname {
  margin-bottom: 10px;
  color: #333;
}

.nft-label {
  color: #28a745;
  margin-bottom: 5px;
}

.nft-image {
  width: 100px;
  height: 100px;
  border-radius: 8px;
  box-shadow: 0 0 5px rgba(0, 0, 0, 0.1);
}

.profile-actions {
  margin-top: 20px;
}

.user-bio h4 {
  margin-top: 20px;
  margin-bottom: 10px;
}

.profile-details {
  margin-top: 20px;
}

.create-post textarea {
  margin-top: 20px;
  width: 100%;
}

.post-item {
  margin-top: 20px;
  padding: 10px;
  background-color: #ffffff;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.user-friends {
  margin-top: 20px;
}

</style>
