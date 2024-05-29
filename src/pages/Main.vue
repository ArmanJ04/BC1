<template>
  <div class="body">
    <div class="container">
      <div class="main-body">
        <div class="row">
          <div class="col-lg-4">
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
                  <h5>Description: {{ user.bio || 'No description' }}</h5>
                  <h5>Address: {{ $store.state.address }}</h5>
                  <div v-if="friends.length >= 1 && !hasNFT">
                    <button @click="getNFT" class="btn btn-light-blue btn-block">Get Blockchain Badge</button>
                  </div>
                </div>
              </div>
              <div class="profile-actions">
                <button @click="refresh" class="btn btn-light-blue btn-block">Update Profile Data</button>
                <button @click="goToEditPage" class="btn btn-light-blue btn-block">Edit Profile</button>
              </div>
            </div>
          </div>
          <div class="col-lg-4">
            <div class="create-post" >
              <h4 class="text-center">Write a Post</h4>
              <textarea v-model="postContent" class="form-control" rows="3" placeholder="What's new?"></textarea>
              <button @click="createPost" class="btn btn-light-blue btn-block">Send</button>
            </div>
          </div>
          <div class="col-lg-4">
            <div class="user-friends">
              <h1 class="text-center">Friends</h1>
              <div v-for="address in friends" :key="address">
                <friend-item :userAddress="address" />
              </div>
            </div>
          </div>
        </div>
        <div class="row">
          <div class="col-lg-12">
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
body {
    margin: 0;
    font-family: Arial, sans-serif;
    background-color: #ffffff; 
}

.container {
    display: flex;
    justify-content: center;
    padding: 20px;
}

.main-body {
    width: 100%;
    display: flex;
    flex-direction: column;
}

.row {
    display: flex;
    justify-content: space-between;
    width: 100%;
}

.col-lg-4 {
    padding: 10px;
}

.user-profile, .create-post, .user-friends {
    background-color: #ffffff; 
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
}

.user-profile {
   
  margin-bottom: 20px;
}

.profile-header {
    display: flex;
    align-items: center;
    justify-content: space-between; 
}

.profile-image .avatar {
    width: 100px;
    height: 100px;
    border-radius: 50%;
    object-fit: cover;
    margin-right: 20px;
}

.profile-info {
    flex-grow: 1; 
}

.nickname {
    margin: 0;
    color: #333; 
}

.nft-label {
    margin: 5px 0 0;
    color: #4caf50; 
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

.user-bio, .profile-details {
    margin-top: 20px;
}

.user-bio h4, .profile-details h5 {
    margin: 0;
    color: #333; 
}

.create-post textarea.form-control {
    width: 100%;
    padding: 10px;
    font-size: 16px;
    border: 1px solid #ccc;
    border-radius: 4px;
    resize: none;
}

button.btn-light-blue {
    background-color: #add8e6; 
    color: white; 
    border: none;
    padding: 10px 20px;
    font-size: 16px;
    cursor: pointer;
    transition: background-color 0.3s ease;
    width: 100%; 
}

.create-post button.btn-light-blue:hover {
    background-color: #8ab8cd; 
}

.user-friends {
    text-align: center;
    background-color: #ffffff; 
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
}

.user-friends h1 {
    margin-bottom: 20px;
    color: #333; 
}

.post-item {
    padding: 10px;
    border-bottom: 1px solid #ccc;
    margin-bottom: 10px;
}
</style>
