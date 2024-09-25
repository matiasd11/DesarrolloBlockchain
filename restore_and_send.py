from cryptos import *
words = "mountain final blue into giggle subject give employ increase unusual wink ranch"
coin = Bitcoin(testnet=True)
wallet = coin.wallet(words)
addr1 = wallet.new_receiving_address()
print("addr1: ", addr1)
addr2 = wallet.new_change_address()
print("address 2: ",addr2)
inputs = coin.unspent(addr1)
print("inputs: ",inputs)
outs = [{'value': 20, 'address': 'miY6dpy1vTk1tVXgLpKQLvXizgEMz4W8Ba'}]
tx = coin.mktx(inputs, outs)
print("tx: ",tx)
tx2 = coin.signall(tx, wallet.privkey(addr1))
print("tx2: ",tx2)
tx3 = serialize(tx2)
print("tx3: ",tx3)
coin.pushtx(tx3)