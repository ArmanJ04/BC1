<template>
    <div class="body">
      <div class="container">
        <div class="main-body">
          <div class="requests-container">
            <h2 class="title">Pending Friend Requests</h2>
            <button @click="getRequests" class="btn btn-green">Refresh</button>
            <div class="requests-list">
              <div v-for="address in requests" :key="address" class="request-item">
                <request-item :userAddress="address" />
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
        requests: []
      };
    },
    methods: {
      ...mapActions({
        getincomingFriendRequests: "getincomingFriendRequests"
      }),
      async getRequests() {
        console.log(this.$store.state.address)
        this.requests = await this.getincomingFriendRequests([this.$store.state.address])
        console.log(this.requests)
      }
    },
    async mounted() {
      this.getRequests()
    },
    watch: {
      'this.requests': 'getRequests'
    }
  };
  </script>
  
  <style scoped>
  .body {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 100vh;
    background-color: #1e1e1e;
    color: #e0e0e0;
  }
  
  .container {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    padding: 20px;
  }
  
  .main-body {
    width: 100%;
    display: flex;
    justify-content: center;
  }
  
  .requests-container {
    background-color: #2e2e2e;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.5);
    text-align: center;
  }
  
  .title {
    font-size: 24px;
    font-weight: bold;
    color: #76ff03;
    margin-bottom: 20px;
  }
  
  .btn-green {
    background-color: #00e676;
    color: #000;
    border: none;
    padding: 10px 20px;
    font-size: 16px;
    border-radius: 5px;
    cursor: pointer;
    transition: background-color 0.3s ease;
    margin-bottom: 20px;
  }
  
  .btn-green:hover {
    background-color: #00c853;
  }
  
  .requests-list {
    display: flex;
    flex-direction: column;
    gap: 15px;
  }
  
  .request-item {
    background-color: #424242;
    border-radius: 8px;
    padding: 15px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
    transition: transform 0.3s ease, box-shadow 0.3s ease;
  }
  
  .request-item:hover {
    transform: translateY(-5px);
    box-shadow: 0 6px 12px rgba(0, 0, 0, 0.5);
  }
  </style>
  