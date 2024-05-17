// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Web3Linkedin {

    struct UserProfile {
        string name;
        string bio;
        string profilePictureCID;
        address[] friends;
        address[] friendRequests;
        address[] incomingFriendRequests;
        bool hasMintedNft;
    }

    struct Post {
        uint256 id;
        address author;
        string content;
    }

    address public owner;
    uint256 public postCounter;

    mapping(address => UserProfile) public users;
    mapping(address => Post[]) public userPosts;

    event FriendRequestSent(address indexed from, address indexed to);
    event FriendRequestAccepted(address indexed from, address indexed to);
    event FriendRequestDeclined(address indexed from, address indexed to);
    event ProfileUpdated(address indexed user);
    event OwnerChanged(address indexed previousOwner, address indexed newOwner);
    event NftStatusUpdated(address indexed user, bool value);
    event PostCreated(uint256 id, address indexed author, string content);

    constructor() {
        owner = msg.sender;
    }

    modifier notAlreadyFriends(address _user) {
        require(!isFriend(msg.sender, _user), "Users are already friends");
        _;
    }

    modifier notDuplicateFriendRequest(address _to) {
        require(!isFriendRequestSent(_to), "Friend request already sent");
        _;
    }

    modifier onlyOwner() {
        require(
            msg.sender == owner,
            "Only contract owner can call this function"
        );
        _;
    }

    modifier onlyMintedNftUser() {
        require(users[msg.sender].hasMintedNft, "User has not minted NFT");
        _;
    }

    modifier onlyRegisteredUser() {
        require(bytes(users[msg.sender].name).length > 0, "User is not registered");
        _;
    }

    function getFriendRequests(address _user)
        external
        view
        returns (address[] memory)
    {
        return users[_user].friendRequests;
    }

    function getincomingFriendRequests(address _user)
        external
        view
        returns (address[] memory)
    {
        return users[_user].incomingFriendRequests;
    }

    function getUserProfile(address _user)
        external
        view
        returns (
            string memory name,
            string memory bio,
            string memory profilePicture,
            address[] memory friends,
            bool hasMintedNft
        )
    {
        UserProfile memory userProfile = users[_user];
        return (
            userProfile.name,
            userProfile.bio,
            userProfile.profilePictureCID,
            userProfile.friends,
            userProfile.hasMintedNft
        );
    }

    function changeOwner(address newOwner) external onlyOwner {
        require(newOwner != address(0), "Invalid new owner address");
        emit OwnerChanged(owner, newOwner);
        owner = newOwner;
    }

    function setNft(address _user, bool _value) external onlyOwner {
        require(!users[_user].hasMintedNft, "NFT status can only be set once");
        users[_user].hasMintedNft = _value;
        emit NftStatusUpdated(_user, _value);
    }

    function createPost(string memory _content) external  onlyRegisteredUser onlyMintedNftUser {
        postCounter++;
        Post memory newPost = Post({
            id: postCounter,
            author: msg.sender,
            content: _content
        });
        userPosts[msg.sender].push(newPost);
        emit PostCreated(postCounter,msg.sender, _content);
    }

    function getUserPosts(address _user) external view returns (Post[] memory) {
        return userPosts[_user];
    }

    function registerProfile(string memory _name) external {
        require(
            bytes(users[msg.sender].name).length == 0,
            "User already registered"
        );
        users[msg.sender] = UserProfile(
            _name,
            "",
            "",
            new address[](0),
            new address[](0),
            new address[](0),
            false
        );
    }


    function updateProfile(
        string memory _name,
        string memory _bio,
        string memory _profilePictureCID
    ) external {
        UserProfile storage user = users[msg.sender];

        if (
            bytes(_name).length > 0 &&
            keccak256(bytes(user.name)) != keccak256(bytes(_name))
        ) {
            user.name = _name;
        }

        if (
            bytes(_bio).length > 0 &&
            keccak256(bytes(user.bio)) != keccak256(bytes(_bio))
        ) {
            user.bio = _bio;
        }

        if (
            bytes(_profilePictureCID).length > 0 &&
            keccak256(bytes(user.profilePictureCID)) !=
            keccak256(bytes(_profilePictureCID))
        ) {
            user.profilePictureCID = _profilePictureCID;
        }

        emit ProfileUpdated(msg.sender);
    }

    function sendFriendRequest(address _to)
        external
        notAlreadyFriends(_to)
        notDuplicateFriendRequest(_to)
    {
        require(msg.sender != _to, "Cannot send friend request to yourself"); // Prevent self-sending
        users[msg.sender].friendRequests.push(_to);
        users[_to].incomingFriendRequests.push(msg.sender); // Add sender to incoming requests of recipient
        emit FriendRequestSent(msg.sender, _to);
    }

    function acceptFriendRequest(address _from) external {
        require(
            isFriendRequestReceived(_from),
            "No friend request from this user"
        );
        users[msg.sender].friends.push(_from);
        users[_from].friends.push(msg.sender);
        removeFriendRequest(_from, msg.sender);
        removeIncomingFriendRequest(msg.sender, _from);
        emit FriendRequestAccepted(_from, msg.sender);
    }

    function declineFriendRequest(address _from) external {
        require(
            isFriendRequestReceived(_from),
            "No friend request from this user"
        );
        removeFriendRequest(_from, msg.sender);
        removeIncomingFriendRequest(msg.sender, _from);
        emit FriendRequestDeclined(_from, msg.sender);
    }

    function removeFriend(address _friend) external {
        require(isFriend(msg.sender, _friend), "User is not a friend");
        removeFriendFromList(msg.sender, _friend);
        removeFriendFromList(_friend, msg.sender);
    }

    function removeFriendFromList(address _user, address _friend) internal {
        for (uint256 i = 0; i < users[_user].friends.length; i++) {
            if (users[_user].friends[i] == _friend) {
                users[_user].friends[i] = users[_user].friends[
                    users[_user].friends.length - 1
                ];
                users[_user].friends.pop();
                break;
            }
        }
    }

    function removeFriendRequest(address _user, address _from) internal {
        for (uint256 i = 0; i < users[_user].friendRequests.length; i++) {
            if (users[_user].friendRequests[i] == _from) {
                users[_user].friendRequests[i] = users[_user].friendRequests[
                    users[_user].friendRequests.length - 1
                ];
                users[_user].friendRequests.pop();
                break;
            }
        }
    }

    function removeIncomingFriendRequest(address _user, address _from)
        internal
    {
        for (
            uint256 i = 0;
            i < users[_user].incomingFriendRequests.length;
            i++
        ) {
            if (users[_user].incomingFriendRequests[i] == _from) {
                users[_user].incomingFriendRequests[i] = users[_user]
                    .incomingFriendRequests[
                        users[_user].incomingFriendRequests.length - 1
                    ];
                users[_user].incomingFriendRequests.pop();
                break;
            }
        }
    }

    function isFriendRequestSent(address _to) internal view returns (bool) {
        for (uint256 i = 0; i < users[msg.sender].friendRequests.length; i++) {
            if (users[msg.sender].friendRequests[i] == _to) {
                return true;
            }
        }
        return false;
    }

    function isFriendRequestReceived(address _from)
        internal
        view
        returns (bool)
    {
        for (uint256 i = 0; i < users[_from].friendRequests.length; i++) {
            if (users[_from].friendRequests[i] == msg.sender) {
                return true;
            }
        }
        return false;
    }

    function isFriend(address _user1, address _user2)
        internal
        view
        returns (bool)
    {
        for (uint256 i = 0; i < users[_user1].friends.length; i++) {
            if (users[_user1].friends[i] == _user2) {
                return true;
            }
        }
        return false;
    }
}
