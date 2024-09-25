from cryptos import *
words = "mountain final blue into giggle subject give employ increase unusual wink ranch"
coin = Bitcoin(testnet=True)
wallet = coin.wallet(words)
addr1 = wallet.new_receiving_address()
print("addr1: ", addr1)
addr2 = wallet.new_change_address()
print("address 2: ",addr2)
inputs = coin.get_unspents(addr1)
print("inputs addr1: ",inputs)
inputs2 = coin.unspent(addr2)
print("inputs addr2: ",inputs2)