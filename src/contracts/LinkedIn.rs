// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Web3Linkedin {
    struct UserProfile {
        name: String,
        bio: String,
        profile_picture_cid: String,
        friends: Vec<Address>,
        friend_requests: Vec<Address>,
        incoming_friend_requests: Vec<Address>,
        has_minted_nft: bool,
    }

    struct Post {
        id: u256,
        author: Address,
        content: String,
    }

    address public owner;
    uint256 public post_counter;

    mapping(address => UserProfile) public users;
    mapping(address => Vec<Post>) public user_posts;

    event FriendRequestSent(address indexed from, address indexed to);
    event FriendRequestAccepted(address indexed from, address indexed to);
    event FriendRequestDeclined(address indexed from, address indexed to);
    event ProfileUpdated(address indexed user);
    event OwnerChanged(address indexed previous_owner, address indexed new_owner);
    event NftStatusUpdated(address indexed user, bool value);
    event PostCreated(uint256 id, address indexed author, string content);

    constructor() {
        owner = msg.sender;
    }

    modifier not_already_friends(address _user) {
        require(!is_friend(msg.sender, _user), "Users are already friends");
        _;
    }

    modifier not_duplicate_friend_request(address _to) {
        require(!is_friend_request_sent(_to), "Friend request already sent");
        _;
    }

    modifier only_owner() {
        require(
            msg.sender == owner,
            "Only contract owner can call this function"
        );
        _;
    }

    modifier only_minted_nft_user() {
        require(users[msg.sender].has_minted_nft, "User has not minted NFT");
        _;
    }

    modifier only_registered_user() {
        require(users[msg.sender].name != "", "User is not registered");
        _;
    }

    function get_friend_requests(address _user)
        external
        view
        returns (Vec<Address>)
    {
        return users[_user].friend_requests;
    }

    function get_incoming_friend_requests(address _user)
        external
        view
        returns (Vec<Address>)
    {
        return users[_user].incoming_friend_requests;
    }

    function get_user_profile(address _user)
        external
        view
        returns (
            String memory name,
            String memory bio,
            String memory profile_picture,
            Vec<Address> memory friends,
            bool has_minted_nft
        )
    {
        UserProfile memory user_profile = users[_user];
        return (
            user_profile.name,
            user_profile.bio,
            user_profile.profile_picture_cid,
            user_profile.friends,
            user_profile.has_minted_nft
        );
    }

    function change_owner(address new_owner) external only_owner {
        require(new_owner != address(0), "Invalid new owner address");
        emit OwnerChanged(owner, new_owner);
        owner = new_owner;
    }

    function set_nft(address _user, bool _value) external only_owner {
        require(!users[_user].has_minted_nft, "NFT status can only be set once");
        users[_user].has_minted_nft = _value;
        emit NftStatusUpdated(_user, _value);
    }

    function create_post(string memory _content) external only_registered_user only_minted_nft_user {
        post_counter++;
        Post memory new_post = Post({
            id: post_counter,
            author: msg.sender,
            content: _content
        });
        user_posts[msg.sender].push(new_post);
        emit PostCreated(post_counter, msg.sender, _content);
    }

    function get_user_posts(address _user) external view returns (Vec<Post> memory) {
        return user_posts[_user];
    }

    function register_profile(string memory _name) external {
        require(
            users[msg.sender].name == "",
            "User already registered"
        );
        users[msg.sender] = UserProfile(
            _name,
            "",
            "",
            Vec<Address>(0),
            Vec<Address>(0),
            Vec<Address>(0),
            false
        );
    }

    function update_profile(
        string memory _name,
        string memory _bio,
        string memory _profile_picture_cid
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
            bytes(_profile_picture_cid).length > 0 &&
            keccak256(bytes(user.profile_picture_cid)) !=
            keccak256(bytes(_profile_picture_cid))
        ) {
            user.profile_picture_cid = _profile_picture
            _cid;
        }

        emit ProfileUpdated(msg.sender);
    }

    function send_friend_request(address _to)
        external
        not_already_friends(_to)
        not_duplicate_friend_request(_to)
    {
        require(msg.sender != _to, "Cannot send friend request to yourself");
        users[msg.sender].friend_requests.push(_to);
        users[_to].incoming_friend_requests.push(msg.sender); 
        emit FriendRequestSent(msg.sender, _to);
    }

    function accept_friend_request(address _from) external {
        require(
            is_friend_request_received(_from),
            "No friend request from this user"
        );
        users[msg.sender].friends.push(_from);
        users[_from].friends.push(msg.sender);
        remove_friend_request(_from, msg.sender);
        remove_incoming_friend_request(msg.sender, _from);
        emit FriendRequestAccepted(_from, msg.sender);
    }

    function decline_friend_request(address _from) external {
        require(
            is_friend_request_received(_from),
            "No friend request from this user"
        );
        remove_friend_request(_from, msg.sender);
        remove_incoming_friend_request(msg.sender, _from);
        emit FriendRequestDeclined(_from, msg.sender);
    }

    function remove_friend(address _friend) external {
        require(is_friend(msg.sender, _friend), "User is not a friend");
        remove_friend_from_list(msg.sender, _friend);
        remove_friend_from_list(_friend, msg.sender);
    }

    function remove_friend_from_list(address _user, address _friend) internal {
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

    function remove_friend_request(address _user, address _from) internal {
        for (uint256 i = 0; i < users[_user].friend_requests.length; i++) {
            if (users[_user].friend_requests[i] == _from) {
                users[_user].friend_requests[i] = users[_user].friend_requests[
                    users[_user].friend_requests.length - 1
                ];
                users[_user].friend_requests.pop();
                break;
            }
        }
    }

    function remove_incoming_friend_request(address _user, address _from)
        internal
    {
        for (
            uint256 i = 0;
            i < users[_user].incoming_friend_requests.length;
            i++
        ) {
            if (users[_user].incoming_friend_requests[i] == _from) {
                users[_user].incoming_friend_requests[i] = users[_user]
                    .incoming_friend_requests[
                        users[_user].incoming_friend_requests.length - 1
                    ];
                users[_user].incoming_friend_requests.pop();
                break;
            }
        }
    }

    function is_friend_request_sent(address _to) internal view returns (bool) {
        for (uint256 i = 0; i < users[msg.sender].friend_requests.length; i++) {
            if (users[msg.sender].friend_requests[i] == _to) {
                return true;
            }
        }
        return false;
    }

    function is_friend_request_received(address _from)
        internal
        view
        returns (bool)
    {
        for (uint256 i = 0; i < users[_from].friend_requests.length; i++) {
            if (users[_from].friend_requests[i] == msg.sender) {
                return true;
            }
        }
        return false;
    }

    function is_friend(address _user1, address _user2)
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
