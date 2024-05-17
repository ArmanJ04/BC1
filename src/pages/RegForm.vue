<template>
    <div class="body">
        <div class="container">
            <div class="row justify-content-center">
                <div class="col-md-8">
                    <form @submit.prevent="submitForm" class="forms-container">
                        <h2 class="title">Sign up</h2>
                        <div class="mb-4">
                            <!-- <label for="name" class="form-label">Name</label> -->
                            <div class="input-group">
                                <input v-model="name" type="text" class="form-control" id="name" placeholder="Name"
                                    name="first_name" required />
                            </div>
                        </div>
                        <div class="mb-4">
                            <button @click="connectWallet" class="btn btn-primary">Connect Metamask</button>
                        </div>
                        <div class="mb-4">
                            <h3>Your address</h3>
                            <p>{{ $store.state.address }}</p>
                        </div>
                        <div class="mb-4">
                            <input type="submit" class="btn btn-primary" value="Sign up" />
                        </div>
                    </form>
                </div>
            </div>
        </div>
    </div>
</template>

	
<script>
import { mapActions } from 'vuex'
export default {
    name: 'RegFormView',
    data() {
        return {
            // selectedFile: null, 
            name: ""
        };
    },
    methods: {
        ...mapActions({
            connectWallet: "connectWallet",
            registerProfile: "registerProfile"
        }),
        async submitForm() {
            try {
                await this.registerProfile([this.name]);
                this.$router.push('/')
            } catch (error) {
                console.error('Error submitting form:', error);
            }
        },
    },
};
</script>
	
<style scoped>
.body {
    background-color: #f8f9fa;
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
}
.container {
    background-color: #ffffff;
    border-radius: 8px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    padding: 40px;
    max-width: 800px;
    width: 100%;
}
.forms-container {
    padding: 20px;
}
.title {
    font-size: 28px;
    font-weight: bold;
    margin-bottom: 30px;
    text-align: center;
}
.input-group {
    display: flex;
    align-items: center;
}
.input-group-text {
    background-color: #ffffff;
    border-right: 0;
    padding: 10px 15px;
}
.form-control {
    padding: 10px 15px;
    flex: 1;
}
.mb-4 {
    margin-bottom: 1.5rem !important;
}
.btn-primary {
    width: 100%;
    padding: 10px 0;
}
</style>
	