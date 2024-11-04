from web3 import Web3
import os
from dotenv import load_dotenv
import time

load_dotenv()
NODE = os.environ.get("NODE")
print(NODE)
w3 = Web3(Web3.HTTPProvider(NODE))
abi = [
		{
			"inputs": [],
			"stateMutability": "nonpayable",
			"type": "constructor"
		},
		{
			"anonymous": False,
			"inputs": [
				{
					"indexed": True,
					"internalType": "address",
					"name": "from",
					"type": "address"
				},
				{
					"indexed": False,
					"internalType": "uint256",
					"name": "amount",
					"type": "uint256"
				}
			],
			"name": "Deposit",
			"type": "event"
		},
		{
			"anonymous": False,
			"inputs": [
				{
					"indexed": True,
					"internalType": "address",
					"name": "to",
					"type": "address"
				},
				{
					"indexed": False,
					"internalType": "uint256",
					"name": "amount",
					"type": "uint256"
				}
			],
			"name": "Withdrawal",
			"type": "event"
		},
		{
			"inputs": [],
			"name": "depositar",
			"outputs": [],
			"stateMutability": "payable",
			"type": "function"
		},
		{
			"inputs": [],
			"name": "getBalance",
			"outputs": [
				{
					"internalType": "uint256",
					"name": "",
					"type": "uint256"
				}
			],
			"stateMutability": "view",
			"type": "function"
		},
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
					"internalType": "uint256",
					"name": "_amount",
					"type": "uint256"
				}
			],
			"name": "retirar",
			"outputs": [],
			"stateMutability": "nonpayable",
			"type": "function"
		}
	]
contract_address = w3.to_checksum_address("0x6Df5f9e9BdaB950a4E6e826067Dd252bEf1a4847")
print(contract_address)
mi_contrato = w3.eth.contract(
 address=contract_address,
 abi=abi
)
print(mi_contrato.functions.getBalance().call())
account = w3.eth.account.from_key(os.environ.get("pk"))

def depositar(monto):
    tx = {
        "from": account.address,
        "value": w3.to_wei(monto, 'ether'),  
        "gas": 200000,
        "gasPrice": w3.eth.gas_price,
        "nonce": w3.eth.get_transaction_count(account.address),
        "chainId": 11155111 
    }
    contract_data = mi_contrato.functions.depositar().build_transaction(tx)
    signed_tx = w3.eth.account.sign_transaction(contract_data , os.environ.get("pk"))
    tx_hash = w3.eth.send_raw_transaction(signed_tx.raw_transaction)
    print(f"Transacción de depósito enviada con hash: {tx_hash.hex()}")
    return tx_hash

def retirar(monto):
    tx = mi_contrato.functions.retirar(w3.to_wei(monto, 'ether')).build_transaction({
        "from": account.address,
        "gas": 200000,
        "gasPrice": w3.eth.gas_price,
        "nonce": w3.eth.get_transaction_count(account.address),
        "chainId": 11155111
    })
    signed_tx = w3.eth.account.sign_transaction(tx , os.environ.get("pk"))
    tx_hash = w3.eth.send_raw_transaction(signed_tx.raw_transaction)
    print(f"Transacción de retiro enviada con hash: {tx_hash.hex()}")
    return tx_hash

#depositar(0.001)  
#retirar(0.0005)