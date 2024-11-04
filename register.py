from web3 import Web3
import os
from dotenv import load_dotenv

load_dotenv()
NODE = os.environ.get("NODE")
w3 = Web3(Web3.HTTPProvider(NODE))

abi = [
    {
        "inputs": [],
        "name": "owner",
        "outputs": [
            {
                "internalType": "address",
                "name": "",
                "type": "address"
            }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "address",
                "name": "",
                "type": "address"
            }
        ],
        "name": "users",
        "outputs": [
            {
                "internalType": "bool",
                "name": "isRegistered",
                "type": "bool"
            },
            {
                "internalType": "string",
                "name": "username",
                "type": "string"
            }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "string",
                "name": "username",
                "type": "string"
            }
        ],
        "name": "register",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "address",
                "name": "userAddress",
                "type": "address"
            }
        ],
        "name": "isUserRegistered",
        "outputs": [
            {
                "internalType": "bool",
                "name": "",
                "type": "bool"
            }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "address",
                "name": "userAddress",
                "type": "address"
            }
        ],
        "name": "getUsername",
        "outputs": [
            {
                "internalType": "string",
                "name": "",
                "type": "string"
            }
        ],
        "stateMutability": "view",
        "type": "function"
    }
]

contract_address = w3.to_checksum_address("0x44861986042725e6591e234424e7C841188a2c33")
mi_contrato = w3.eth.contract(address=contract_address, abi=abi)

account = w3.eth.account.from_key(os.environ.get("pk"))


tx = {
    "nonce": w3.eth.get_transaction_count(account.address),
    "gas": 200000,
    "gasPrice": w3.eth.gas_price,
    "value": 0,
    "chainId": 11155111,
}


def register_user(username):
    contract_data = mi_contrato.functions.register(username).build_transaction(tx)
    signed_txn = w3.eth.account.sign_transaction(contract_data, os.environ.get("pk"))
    txn_hash = w3.eth.send_raw_transaction(signed_txn.raw_transaction)
    print(f"Transacción para registrar enviada: {txn_hash.hex()}")

def check_user_registration(user_address):
    is_registered = mi_contrato.functions.isUserRegistered(user_address).call()
    return is_registered


def get_username(user_address):
    username = mi_contrato.functions.getUsername(user_address).call()
    return username


username = "user123"  
register_user(username)


user_address = account.address 
is_registered = check_user_registration(user_address)
print(f"¿El usuario está registrado? {is_registered}")

if is_registered:
    username = get_username(user_address)
    print(f"Nombre de usuario: {username}")
