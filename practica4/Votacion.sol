// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

contract Voting {
    struct Candidate {
        string name;
        uint256 voteCount;
    }

    address public owner;
    mapping(address => bool) public hasVoted;
    Candidate[] public candidates;

    // Evento que se dispara cuando alguien vota
    event Vote(address indexed voter, uint256 indexed candidateId);

    // Modificador para restringir ciertas acciones al owner
    modifier onlyOwner() {
        require(msg.sender == owner, "Only the owner can execute this function.");
        _;
    }

    constructor(string[] memory candidateNames) {
        owner = msg.sender; // El owner es quien despliega el contrato

        // Agregar candidatos al arreglo de candidatos
        for (uint256 i = 0; i < candidateNames.length; i++) {
            candidates.push(Candidate({
                name: candidateNames[i],
                voteCount: 0
            }));
        }
    }

    // Función para votar por un candidato (cada participante solo puede votar una vez)
    function vote(uint256 candidateId) public {
        require(!hasVoted[msg.sender], "You have already voted.");
        require(candidateId < candidates.length, "Invalid candidate ID.");

        candidates[candidateId].voteCount += 1; // Incrementar el voto del candidato
        hasVoted[msg.sender] = true; // Registrar que el votante ha votado

        emit Vote(msg.sender, candidateId); // Emitir evento de votación
    }

    // Función para declarar al ganador basado en los votos
    function declareWinner() public view onlyOwner returns (string memory winnerName) {
        uint256 winningVoteCount = 0;
        uint256 winnerIndex = 0;

        for (uint256 i = 0; i < candidates.length; i++) {
            if (candidates[i].voteCount > winningVoteCount) {
                winningVoteCount = candidates[i].voteCount;
                winnerIndex = i;
            }
        }

        winnerName = candidates[winnerIndex].name;
    }

    // Función para obtener el total de candidatos
    function getTotalCandidates() public view returns (uint256) {
        return candidates.length;
    }

    // Función para obtener los detalles de un candidato por su ID
    function getCandidate(uint256 candidateId) public view returns (string memory name, uint256 voteCount) {
        require(candidateId < candidates.length, "Invalid candidate ID.");
        Candidate memory candidate = candidates[candidateId];
        return (candidate.name, candidate.voteCount);
    }
}
