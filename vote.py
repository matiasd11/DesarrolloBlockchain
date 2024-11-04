from web3 import Web3
import os
from dotenv import load_dotenv
load_dotenv()
NODE = os.environ.get("NODE")
print(NODE)
w3 = Web3(Web3.HTTPProvider(NODE))
abi = [
		{
			"inputs": [
				{
					"internalType": "string[]",
					"name": "candidateNames",
					"type": "string[]"
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
					"name": "voter",
					"type": "address"
				},
				{
					"indexed": True,
					"internalType": "uint256",
					"name": "candidateId",
					"type": "uint256"
				}
			],
			"name": "Vote",
			"type": "event"
		},
		{
			"inputs": [
				{
					"internalType": "uint256",
					"name": "",
					"type": "uint256"
				}
			],
			"name": "candidates",
			"outputs": [
				{
					"internalType": "string",
					"name": "name",
					"type": "string"
				},
				{
					"internalType": "uint256",
					"name": "voteCount",
					"type": "uint256"
				}
			],
			"stateMutability": "view",
			"type": "function"
		},
		{
			"inputs": [],
			"name": "declareWinner",
			"outputs": [
				{
					"internalType": "string",
					"name": "winnerName",
					"type": "string"
				}
			],
			"stateMutability": "view",
			"type": "function"
		},
		{
			"inputs": [
				{
					"internalType": "uint256",
					"name": "candidateId",
					"type": "uint256"
				}
			],
			"name": "getCandidate",
			"outputs": [
				{
					"internalType": "string",
					"name": "name",
					"type": "string"
				},
				{
					"internalType": "uint256",
					"name": "voteCount",
					"type": "uint256"
				}
			],
			"stateMutability": "view",
			"type": "function"
		},
		{
			"inputs": [],
			"name": "getTotalCandidates",
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
			"inputs": [
				{
					"internalType": "address",
					"name": "",
					"type": "address"
				}
			],
			"name": "hasVoted",
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
					"name": "candidateId",
					"type": "uint256"
				}
			],
			"name": "vote",
			"outputs": [],
			"stateMutability": "nonpayable",
			"type": "function"
		}
	]
contract_address = w3.to_checksum_address("0xDE47a32aBF5EcD5fa474CE9E9B84c21B0e104F8c")
print(contract_address)
mi_contrato = w3.eth.contract(
 address=contract_address,
 abi=abi
)
print(mi_contrato.functions.getTotalCandidates().call())
account = w3.eth.account.from_key(os.environ.get("pk"))

voter_address = "0xE3526F7FB453C1201Fc3a256bE0ee5B27AdBa97A" 
ha_votado = mi_contrato.functions.hasVoted(w3.to_checksum_address(voter_address)).call()
print(f"El votante con dirección {voter_address} ha votado: {ha_votado}")

candidate_id = 0  
candidato = mi_contrato.functions.getCandidate(candidate_id).call()
print(f"Nombre del candidato: {candidato[0]}, Votos recibidos: {candidato[1]}")

def votar(candidate_id):
    tx = mi_contrato.functions.vote(candidate_id).build_transaction({
        'from': account.address,
        'gas': 200000,
        'gasPrice': w3.eth.gas_price,
        'nonce': w3.eth.get_transaction_count(account.address),
        'chainId': 11155111 
    })

    signed_tx = w3.eth.account.sign_transaction(tx, os.environ.get("pk"))
    tx_hash = w3.eth.send_raw_transaction(signed_tx.raw_transaction)
    print(f"Voto enviado. Hash de la transacción: {tx_hash.hex()}")
    return tx_hash

#votar(0)  
owner_address = mi_contrato.functions.owner().call()  

if owner_address.lower() == account.address.lower():
    print("Estás llamando a la función desde el owner del contrato.")
    
    ganador = mi_contrato.functions.declareWinner().call()
    print(f"Nombre del candidato ganador actual: {ganador}")
else:
    print("Error: La cuenta que está llamando a la función no es el owner del contrato.")

