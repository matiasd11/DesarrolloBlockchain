from brownie import Voting, accounts, network, config


def deploy():
    """
    Despliega el contrato Voting con una lista de candidatos predefinidos.
    """
    account = accounts.add(config["deployer_a"])

    candidate_names = ["pepe", "juan", "carlos"]

    voting = Voting.deploy(
        candidate_names,  
        {"from": account}
    )

    print(f"Contract deployed to {voting.address}")
    return voting

#0x7a2E6f39028D2e5E03808732E0EEe4Fe07C5499A

def verify():
    """
    Verifica el contrato desplegado en Etherscan u otro explorador compatible.
    """
    voting = Voting[-1]

    print(f"Verifying contract at {voting.address}")
    result = Voting.publish_source(voting)
    print(f"Verification result: {result}")