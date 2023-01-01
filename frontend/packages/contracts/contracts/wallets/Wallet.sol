// SPDX-License-Identifier: Apache-2.0
pragma solidity >=0.8.0 <0.9.0;

// import "@openzeppelin/contracts/utils/introspection/ERC165.sol";

/**
* 
* @notice Base interface for building wallets
* @title WalletLib
*/
library WalletLib {
    struct Transaction {
        address to;
        uint value;
        bytes data;
        bool executed;
        uint numConfirmations;
    }
}

/**
* 
* @notice Base interface for building wallets
* @title IWallet
*/
interface IWallet {
    /**
    * @dev Emitted when a transaction reaches completion
    */
    event ConfirmTransaction(address indexed owner, uint indexed txIndex);
    /**
    * @dev Emitted when a deposit is submitted
    */
    event Deposit(address indexed sender, uint amount, uint balance);
    /**
    * @dev Emitted when going to execute a transaction
    */
    event ExecuteTransaction(address indexed owner, uint indexed txIndex);
    /**
    * @dev Emitted when a user wishes to revoke a prompt
    */
    event RevokeConfirmation(address indexed owner, uint indexed txIndex);
    /**
    * @dev Emitted when the user sumbits a transaction
    */
    event SubmitTransaction(
        address indexed owner,
        uint indexed txIndex,
        address indexed to,
        uint value,
        bytes data
    );
    
}