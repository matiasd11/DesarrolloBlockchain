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
			"inputs": [
				{
					"internalType": "address[]",
					"name": "_whitelistedAddresses",
					"type": "address[]"
				}
			],
			"stateMutability": "nonpayable",
			"type": "constructor"
		},
		{
			"anonymous": False,
			"inputs": [
				{
					"indexed": True,
					"internalType": "address",
					"name": "by",
					"type": "address"
				},
				{
					"indexed": False,
					"internalType": "int256",
					"name": "newValue",
					"type": "int256"
				}
			],
			"name": "CounterModified",
			"type": "event"
		},
		{
			"inputs": [
				{
					"internalType": "address",
					"name": "_address",
					"type": "address"
				}
			],
			"name": "addToWhitelist",
			"outputs": [],
			"stateMutability": "nonpayable",
			"type": "function"
		},
		{
			"inputs": [],
			"name": "counter",
			"outputs": [
				{
					"internalType": "int256",
					"name": "",
					"type": "int256"
				}
			],
			"stateMutability": "view",
			"type": "function"
		},
		{
			"inputs": [],
			"name": "decrement",
			"outputs": [],
			"stateMutability": "nonpayable",
			"type": "function"
		},
		{
			"inputs": [],
			"name": "getCounter",
			"outputs": [
				{
					"internalType": "int256",
					"name": "",
					"type": "int256"
				}
			],
			"stateMutability": "view",
			"type": "function"
		},
		{
			"inputs": [],
			"name": "increment",
			"outputs": [],
			"stateMutability": "nonpayable",
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
					"internalType": "address",
					"name": "_address",
					"type": "address"
				}
			],
			"name": "removeFromWhitelist",
			"outputs": [],
			"stateMutability": "nonpayable",
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
			"name": "whitelist",
			"outputs": [
				{
					"internalType": "bool",
					"name": "",
					"type": "bool"
				}
			],
			"stateMutability": "view",
			"type": "function"
		}
	]

contract_address = w3.to_checksum_address("0xA5C455d0632Fd22635FEF5D0cF88bC903607EB56")
print(contract_address)
mi_contrato = w3.eth.contract(
 address=contract_address,
 abi=abi
)
print(mi_contrato.functions.getCounter().call())
account = w3.eth.account.from_key(os.environ.get("pk"))
tx = {
 "nonce":w3.eth.get_transaction_count (account.address),
 "gas":200000,
 "gasPrice" :w3.eth.gas_price,
 "value":0 ,
 "chainId" :11155111,
}


def increment_counter():
    contract_data = mi_contrato.functions.increment().build_transaction(tx)
    signed_txn = w3.eth.account.sign_transaction(contract_data, os.environ.get("pk"))
    txn_hash = w3.eth.send_raw_transaction(signed_txn.raw_transaction)
    print(f"Transacción para incrementar enviada: {txn_hash.hex()}")

def decrement_counter():
    contract_data = mi_contrato.functions.decrement().build_transaction(tx)
    signed_txn = w3.eth.account.sign_transaction(contract_data, os.environ.get("pk"))
    txn_hash = w3.eth.send_raw_transaction(signed_txn.raw_transaction)
    print(f"Transacción para decrementar enviada: {txn_hash.hex()}")

def agregar_a_whitelist(direccion):
    account = w3.eth.account.from_key(os.environ.get("pk"))
    tx = {
        "nonce": w3.eth.get_transaction_count(account.address),
        "gas": 200000,
        "gasPrice": w3.eth.gas_price,
        "value": 0,
        "chainId": 11155111,
    }
    contract_data = mi_contrato.functions.addToWhitelist(direccion).build_transaction(tx)
    signed_txn = w3.eth.account.sign_transaction(contract_data, os.environ.get("pk"))
    txn_hash = w3.eth.send_raw_transaction(signed_txn.raw_transaction)
    print(f"Transacción enviada para agregar a whitelist: {txn_hash.hex()}")

def remover_de_whitelist(direccion):
    account = w3.eth.account.from_key(os.environ.get("pk"))
    tx = {
        "nonce": w3.eth.get_transaction_count(account.address),
        "gas": 200000,
        "gasPrice": w3.eth.gas_price,
        "value": 0,
        "chainId": 11155111,
    }
    contract_data = mi_contrato.functions.removeFromWhitelist(direccion).build_transaction(tx)
    signed_txn = w3.eth.account.sign_transaction(contract_data, os.environ.get("pk"))
    txn_hash = w3.eth.send_raw_transaction(signed_txn.raw_transaction)
    print(f"Transacción enviada para remover de whitelist: {txn_hash.hex()}")

event_filter = mi_contrato.events.CounterModified.create_filter(from_block='latest')

def listen_to_events():
    print("Escuchando eventos...")
    while True:
        for event in event_filter.get_new_entries():
            event_data = event['args']
            modificador = event_data['by']
            nuevo_valor = event_data['newValue']
            
            print(f"Evento CounterModified capturado:")
            print(f"Modificado por: {modificador}")
            print(f"Nuevo valor del contador: {nuevo_valor}")
        
        time.sleep(2)

decrement_counter()

listen_to_events()