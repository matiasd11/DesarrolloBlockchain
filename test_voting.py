from brownie import Voting, accounts, config, reverts
import pytest


@pytest.fixture
def voting():
    account = accounts[0]
    candidates = ["juan", "pepe", "carlos"]
    voting_contract = Voting.deploy(candidates, {"from": account})
    return voting_contract


def test_owner_is_deployer(voting):
    assert voting.owner() == accounts[0]


def test_initial_candidates(voting):
    assert voting.getTotalCandidates() == 3
    assert voting.getCandidate(0) == ("juan", 0)
    assert voting.getCandidate(1) == ("pepe", 0)
    assert voting.getCandidate(2) == ("carlos", 0)


def test_vote(voting):
    voter = accounts[1]
    voting.vote(0, {"from": voter})
    assert voting.getCandidate(0)[1] == 1
    assert voting.hasVoted(voter)


def test_double_vote(voting):
    voter = accounts[1]
    voting.vote(0, {"from": voter})
    with reverts("You have already voted."):
        voting.vote(1, {"from": voter})


def test_invalid_candidate(voting):
    voter = accounts[1]
    with reverts("Invalid candidate ID."):
        voting.vote(3, {"from": voter})


def test_declare_winner(voting):
    voter1 = accounts[1]
    voter2 = accounts[2]
    voting.vote(0, {"from": voter1})
    voting.vote(0, {"from": voter2})
    assert voting.declareWinner() == "juan"


def test_get_candidate_details(voting):
    assert voting.getCandidate(1) == ("pepe", 0)

