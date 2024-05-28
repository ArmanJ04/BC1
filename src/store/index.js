import { createStore } from 'vuex'
import { ABI } from "@/contracts/Web3Linkedin.abi.js"
import { bytecode } from "@/contracts/Web3Linkedin.bin.js"
import { NFTABI } from '@/contracts/MERC721.abi';
import { NFTbytecode } from '@/contracts/MERC721.bin';

const axios = require("axios");

const PinataJWT = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySW5mb3JtYXRpb24iOnsiaWQiOiIxYzUxOTRhNS1mYjVjLTQ3NDktYTk3ZS1hZGI5MGQ5MGZmYzkiLCJlbWFpbCI6Im11YWZpa2RAZ21haWwuY29tIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsInBpbl9wb2xpY3kiOnsicmVnaW9ucyI6W3siaWQiOiJGUkExIiwiZGVzaXJlZFJlcGxpY2F0aW9uQ291bnQiOjF9LHsiaWQiOiJOWUMxIiwiZGVzaXJlZFJlcGxpY2F0aW9uQ291bnQiOjF9XSwidmVyc2lvbiI6MX0sIm1mYV9lbmFibGVkIjpmYWxzZSwic3RhdHVzIjoiQUNUSVZFIn0sImF1dGhlbnRpY2F0aW9uVHlwZSI6InNjb3BlZEtleSIsInNjb3BlZEtleUtleSI6ImZlYmNjY2NiNGMyOGNlZjQ4YzFiIiwic2NvcGVkS2V5U2VjcmV0IjoiNTIzMmRiNjE5ZjUyZjFiZDE0ZTk1OTg4MjQ4NDA0M2NlZTM4NzZiNmI5OGU3ODhlODEzOGJkOTQzNjRhMGYzNSIsImlhdCI6MTcxNTc2NzU0MX0.-npB0nQDZKCQbIZo1irPX6IlY6-M26mT1DR7h86xRDM";
const ethers = require('ethers')
let provider = new ethers.providers.JsonRpcProvider("https://eth-sepolia.g.alchemy.com/v2/UqJsOz1IQnRrGqV9bh7Q7ziNR2rN7Pi7")

export default createStore({
    state: {
        address: "",
        chainId: "",
        chain: "",
        deployHash: "",
        contractAddress: "0x8c5ce0D763e0cC799Afdb92477dDe77520Ba7E00",
        isConnected: false,
        username: "",
        userBio: "",
        userProfilePicture: "",
        contest: {},
        ipfsHash: "",
        pinataImageUrl: ""
    },
    getters: {
    },
    mutations: {
    },
    actions: {
        async connectWallet({ state }) {
            if (typeof window.ethereum !== 'undefined') {
                console.log("Ethereum client installed!")
                if (ethereum.isMetaMask === true) {
                    console.log("Metamask installed!")
                    if (ethereum.isConnected() !== true) {
                        console.log("Metamask is not connected!")
                        await ethereum.enable()
                        state.isConnected = true;
                        state.buttonText = 'Connected';
                    }
                    console.log("Metamask connected!")
                    state.isConnected = true;
                    state.buttonText = 'Connected';
                }
                else {
                    alert("Metamask is not installed!")
                }
            }
            else {
                alert("Ethereum client is not installed!")
            }

            ethereum.on('disconnect', () => {
                state.isConnected = false;
                state.buttonText = 'Connect MetaMask';
            });
            provider = new ethers.providers.Web3Provider(ethereum)
            await ethereum.request({ method: "eth_requestAccounts" })
                .then(accounts => {
                    state.address = ethers.utils.getAddress(accounts[0])
                    state.signer = provider.getSigner()
                    console.log(`Account ${state.address} connected`)
                    alert("Connected!")
                })
            state.chainId = await window.ethereum.request({ method: 'eth_chainId' });
            console.log("chainId: ", state.chainId)
            if (state.chainId == "0x1") {
                state.chain = "mainnet"
            }
            else if (state.chainId == "0x5") {
                state.chain = "goerli"
                provider = new ethers.providers.JsonRpcProvider("https://eth-goerli.g.alchemy.com/v2/TS8hjejOOd_2UNj46exSTVtqS7-JxYrT")
                state.contest = new ethers.Contract(state.contractAddress, ABI, provider)
            }
            else if (state.chainId == "0x539") {
                state.chain = "ganache"
                provider = new ethers.providers.JsonRpcProvider("HTTP://127.0.0.1:7545")
                state.contest = new ethers.Contract(state.contractAddress, ABI, provider)
            }
            else if (state.chainId == "0x13881") {
                state.chain = "mumbai"
                provider = new ethers.providers.JsonRpcProvider("https://polygon-mumbai.g.alchemy.com/v2/wusrTgSFSsScFKTK6Nqa5rFoisfYXjPW")
                state.contest = new ethers.Contract(state.contractAddress, ABI, provider)
            }
            else if (state.chainId == "0xaa36a7") {
                state.chain = "sepolia"
                provider = new ethers.providers.JsonRpcProvider("https://eth-sepolia.g.alchemy.com/v2/UqJsOz1IQnRrGqV9bh7Q7ziNR2rN7Pi7")
                state.contest = new ethers.Contract(state.contractAddress, ABI, provider)
            }

            ethereum.on('accountsChanged', (accounts) => {
                state.address = ethers.utils.getAddress(accounts[0])
                state.signer = provider.getSigner()
                console.log(`accounts changed to ${state.address}`)
            })

            ethereum.on('chainChanged', async (chainId) => {
                provider = new ethers.providers.Web3Provider(ethereum)
                state.chainId = await window.ethereum.request({ method: 'eth_chainId' });
                console.log(`chainId changed to ${state.chainId}`)

                if (state.chainId == "0x1") {
                    state.chain = "mainnet"
                    alert(`chain changed to ${state.chain}`)
                }
                else if (state.chainId == "0x5") {
                    state.chain = "goerli"
                    provider = new ethers.providers.JsonRpcProvider("https://eth-goerli.g.alchemy.com/v2/TS8hjejOOd_2UNj46exSTVtqS7-JxYrT")
                    alert(`chain changed to ${state.chain}`)

                }
                else if (state.chainId == "0x539") {
                    state.chain = "ganache"
                    provider = new ethers.providers.JsonRpcProvider("HTTP://127.0.0.1:7545")
                    alert(`chain changed to ${state.chain}`)

                }
                else if (state.chainId == "0x13881") {
                    state.chain = "mumbai"
                    provider = new ethers.providers.JsonRpcProvider("https://polygon-mumbai.g.alchemy.com/v2/wusrTgSFSsScFKTK6Nqa5rFoisfYXjPW")
                    alert(`chain changed to ${state.chain}`)
                }
                else if (state.chainId == "0xaa36a7") {
                    state.chain = "sepolia"
                    provider = new ethers.providers.JsonRpcProvider("https://eth-sepolia.g.alchemy.com/v2/UqJsOz1IQnRrGqV9bh7Q7ziNR2rN7Pi7")
                    alert(`chain changed to ${state.chain}`)
                }
            })
        },
        async uploadFileToPinata({ state }, args) {
            try {
                const [file] = args;
                console.log('File:', file);

                const formData = new FormData();
                formData.append('file', file);
                console.log('FormData after file append:', formData);

                const pinataMetadata = JSON.stringify({
                    name: "File name",
                });
                formData.append("pinataMetadata", pinataMetadata);
                console.log('FormData after pinataMetadata append:', formData);

                const pinataOptions = JSON.stringify({
                    cidVersion: 0,
                });
                formData.append("pinataOptions", pinataOptions);
                console.log('FormData after pinataOptions append:', formData);

                formData.forEach((value, key) => {
                    console.log(`${key}: ${value}`);
                });

                const res = await axios.post("https://api.pinata.cloud/pinning/pinFileToIPFS", formData, {
                    headers: {
                        'Content-Type': 'multipart/form-data',
                        'pinata_api_key': "febccccb4c28cef48c1b",
                        'pinata_secret_api_key': "5232db619f52f1bd14e959882484043cee3876b6b98e788e8138bd94364a0f35",
                    },
                });

                console.log('Response:', res);
                state.ipfsHash = res.data.IpfsHash;
                return res.data.IpfsHash;
            } catch (error) {
                console.error("Error uploading file to Pinata:", error);
                throw error;
            }
        },
        async getImageFromPinata({ state }, hash) {
            try {
                const url = `https://rose-decisive-louse-962.mypinata.cloud/ipfs/${hash}`
                return url
            } catch (error) {
                console.error('Error fetching image from Pinata:', error);
            }
        },
        async registerProfile({ state }, args) {
            const [name] = args

            const iContract = new ethers.utils.Interface(ABI)
            const data = iContract.encodeFunctionData("registerProfile", [name])

            const txHash = await window.ethereum.request({
                method: "eth_sendTransaction",
                params: [{
                    from: state.address,
                    to: state.contractAddress,
                    data: data
                }]
            })
            console.log(`Tx hash: ${txHash}`)

            console.log("Registered succesfully!")
        },
        async updateProfile({ state }, args) {
            const [name, bio, ipfsHash] = args

            try {
                const iContract = new ethers.utils.Interface(ABI)
                const data = iContract.encodeFunctionData("updateProfile", [name, bio, ipfsHash])
                const txHash = await window.ethereum.request({
                    method: 'eth_sendTransaction',
                    params: [{
                        from: state.address,
                        to: state.contractAddress,
                        data: data,
                    }],
                });

                console.log(`Tx hash: ${txHash}`);
                console.log('Profile changed successfully!');
            } catch (error) {
                console.error('Error updating profile:', error);
            }
        },
        async setNft({state}, args){
            const[address, value] = args

            try {
                const iContract = new ethers.utils.Interface(ABI)
                const data = iContract.encodeFunctionData("setNft", [address, value])
                const txHash = await window.ethereum.request({
                    method: 'eth_sendTransaction',
                    params: [{
                        from: "0x4F9ae982818340D29E34994bAedf128C07e42E2f",
                        to: state.contractAddress,
                        data: data,
                    }],
                });

                console.log(`Tx hash: ${txHash}`);
                console.log('hasMintedNft set successfully!');
            } catch (error) {
                console.error('Error:', error);
            }
        },
        async createPost({state}, args){
            const [content] = args
            try{
                const iContract = new ethers.utils.Interface(ABI)
                const data = iContract.encodeFunctionData("createPost", [content])

                const txHash = await window.ethereum.request({
                    method: 'eth_sendTransaction',
                    params: [{
                        from: state.address,
                        to: state.contractAddress,
                        data: data,
                    }],
                });

                console.log(`Tx hash: ${txHash}`);
                console.log('Post created successfully!');
            } catch(error){
                console.log("Error:", error)
            }
        },
        async getUserPosts({state}, args){
            const[address] = args
            const posts = await state.contest.getUserPosts(address)

            return posts
        },
        async getUserProfile({ state, dispatch }, address) {
            console.log(address)
            console.log("Store")
            const [username, userBio, userProfilePicture, userFriends, hasMintedNft] = await state.contest.getUserProfile(address[0])
            console.log(userProfilePicture)
            let url = await dispatch("getImageFromPinata", userProfilePicture)
            
            const user = {
                address: address[0],
                name: username,
                bio: userBio,
                image: url,
                friends: userFriends,
                hasMintedNft: hasMintedNft
            }
            return (user)
        },
        async sendFriendRequest({ state, dispatch }, address) {
            const iContract = new ethers.utils.Interface(ABI)
            const data = iContract.encodeFunctionData("sendFriendRequest", [address])

            const txHash = await window.ethereum.request({
                method: "eth_sendTransaction",
                params: [{
                    from: state.address,
                    to: state.contractAddress,
                    data: data
                }]
            })
            console.log(`Tx hash: ${txHash}`)

            console.log("Friend request sent succesfully!")
        },
        async getFriendRequests({ state }, address) {
            let friendRequests = []
            friendRequests = await state.contest.getFriendRequests(address[0])
            console.log(friendRequests)
            return friendRequests
        },
        async getincomingFriendRequests({ state }, address) {
            console.log(address)
            let incomingFriendRequests = []
            incomingFriendRequests = await state.contest.getincomingFriendRequests(address[0])
            console.log(incomingFriendRequests)
            return incomingFriendRequests
        },
        async acceptFriendRequest({ state, dispatch }, address) {
            const iContract = new ethers.utils.Interface(ABI)
            const data = iContract.encodeFunctionData("acceptFriendRequest", [address[0]])

            const txHash = await window.ethereum.request({
                method: "eth_sendTransaction",
                params: [{
                    from: state.address,
                    to: state.contractAddress,
                    data: data
                }]
            })
            console.log(`Tx hash: ${txHash}`)

            console.log("Friend accepted succesfully!")
        },
        async declineFriendRequest({ state, dispatch }, address) {
            const iContract = new ethers.utils.Interface(ABI)
            const data = iContract.encodeFunctionData("declineFriendRequest", [address[0]])

            const txHash = await window.ethereum.request({
                method: "eth_sendTransaction",
                params: [{
                    from: state.address,
                    to: state.contractAddress,
                    data: data
                }]
            })
            console.log(`Tx hash: ${txHash}`)

            console.log("Friend declined succesfully!")
        },
        async removeFriend({ state, dispatch }, address) {
            const iContract = new ethers.utils.Interface(ABI)
            const data = iContract.encodeFunctionData("removeFriend", [address[0]])

            const txHash = await window.ethereum.request({
                method: "eth_sendTransaction",
                params: [{
                    from: state.address,
                    to: state.contractAddress,
                    data: data
                }]
            })
            console.log(`Tx hash: ${txHash}`)

            console.log("Friend removed succesfully!")
        },
        async getTOPWEB3({ state }, args) {
            const [address, friends] = args

            const privateKey = "4cf4e7ac3a513df3d3d128f465440d83248874bea3c9c75391cd4c0e6c939350"
            const wallet = new ethers.Wallet(privateKey, provider);
            const contract = new ethers.Contract("0x8dE238A81042E99FFCb25666f281429D1EAf59F6", NFTABI, wallet);

            const data = contract.interface.encodeFunctionData("mint", [address, friends])

            const tx = await wallet.sendTransaction({
                to: "0x8dE238A81042E99FFCb25666f281429D1EAf59F6",
                data: data,
            });

            const receipt = await tx.wait();
            console.log("Transaction mined:", receipt.transactionHash);

            console.log("NFT Minted succesfully!")
        },
        async getBalanceNFT({ state }, args) {
            console.log(args)
            const contract = new ethers.Contract("0x8dE238A81042E99FFCb25666f281429D1EAf59F6", NFTABI, provider);
            console.log(contract)
            const balance = await contract.balanceOf(args[0])
            console.log(balance)
            if (balance == 0) {
                return false
            }
            else {
                return true
            }
        }
    },
    modules: {
    }
})
