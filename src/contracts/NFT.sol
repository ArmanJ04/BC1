// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

interface IERC165 {
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
}

interface IERC721Receiver {
    function onERC721Received(address operator, address from, uint256 tokenId, bytes calldata data) external returns (bytes4);
}

interface IERC721 {
    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);

    function balanceOf(address owner) external view returns (uint256 balance);
    function ownerOf(uint256 tokenId) external view returns (address owner);
}

interface IERC721Metadata {
    function name() external view returns (string memory);
    function symbol() external view returns (string memory);
    function tokenURI(uint256 tokenId) external view returns (string memory);
}

abstract contract ERC165 is IERC165 {
    function supportsInterface(bytes4 interfaceId) public view virtual override returns (bool) {
        return interfaceId == type(IERC165).interfaceId;
    }
}


contract MERC721 is IERC721, IERC721Metadata, ERC165{
    //Переменные контракта
    uint256 tokenId;
    string _name;
    string _symbol;
    string baseUri;
    address owner;
    uint256 public constant REQUIRED_FRIENDS = 2;
    //Мэппинги для хранения информации о балансах, адресов токенов
    mapping(uint256 => address) owners;
    mapping(address => uint256) balances;
    
    event Fallback(address caller, uint256 time);
    event NFTMinted(address indexed to, uint256 tokenId);
    //Конструктор
    constructor(string memory baseUri_){
        owner = msg.sender;
        _name = "TOPWEB3";
        _symbol = "TB3";
        baseUri = baseUri_;
    }
    
    fallback() external {
        emit Fallback(msg.sender, block.timestamp);
    }

    // функция эмиссии токенов
    function mint(address to, uint256 providedFriends) external returns (uint256) {
        require(msg.sender == owner, "ERC721: You are not the owner");
        require(providedFriends >= REQUIRED_FRIENDS, "ERC721: User does not have the required number of friends");

        tokenId++;
        owners[tokenId] = to;
        balances[to]++;
        emit Transfer(address(0), to, tokenId);

        emit NFTMinted(to, tokenId);

        return tokenId;
    }

   // функция проверки поддерживаемых интерфейсов
    function supportsInterface(bytes4 interfaceId) public view override returns (bool) {
        return
            interfaceId == type(IERC721).interfaceId ||
            interfaceId == type(IERC721Metadata).interfaceId ||
            super.supportsInterface(interfaceId);
    }

    // возвращает название токена
    function name() public view returns (string memory){
        return _name;
    }

    // возвращает символа токена
    function symbol() public view returns (string memory){
        return _symbol;
    }

    // возращает id Токена
    function getTokenId() public view returns (uint256) {
        return tokenId;
    }

    // возвращает URI токена по его id
    function tokenURI(uint256 _tokenId) public view returns (string memory) {
        require(owners[_tokenId] != address(0), "ERC721: URI query for nonexistent token");
        return string(abi.encodePacked(baseUri, toString(_tokenId)));
    }

    // возвращает баланса аккаунта по его адресу
    function balanceOf(address _owner) external view returns (uint256) {
        return balances[_owner];
    }

    // возвращает адрес владельца токена по его id
    function ownerOf(uint256 _tokenId) external view returns (address) {
        return owners[_tokenId];
    }

    // Функция для перевода числа в строку
    function toString(uint256 value) internal pure returns(string memory) {
        uint256 temp = value;
        uint256 digits;
        do {
            digits++;
            temp /= 10;
        } while (temp != 0);
        bytes memory buffer = new bytes(digits);
        while (value != 0) {
            digits -= 1;
            buffer[digits] = bytes1(uint8(48 + uint256(value % 10)));
            value /= 10;
        }
        return string(buffer);
    }

}