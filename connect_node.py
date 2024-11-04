from web3 import Web3
import os
from dotenv import load_dotenv

load_dotenv()
NODE = os.environ.get("NODE")
print(NODE)
w3 = Web3(Web3.HTTPProvider(NODE))
print(w3.is_connected())
