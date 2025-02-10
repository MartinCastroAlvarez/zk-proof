// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

library Pairing {
    // The prime q for the BN254/BN256 curve:
    uint256 constant FIELD_MODULUS =
        21888242871839275222246405745257275088548364400416034343698204186575808495617;

    struct G1Point {
        uint256 x;
        uint256 y;
    }

    struct G2Point {
        // (x[0], x[1]), (y[0], y[1]) in Fp2
        uint256[2] x;
        uint256[2] y;
    }

    // Returns the point at infinity (additive identity) in G1.
    function P1() internal pure returns (G1Point memory) {
        return G1Point(0, 0);
    }

    // Returns the point at infinity in G2.
    function P2() internal pure returns (G2Point memory) {
        return G2Point([uint256(0), uint256(0)], [uint256(0), uint256(0)]);
    }

    /**
     * @dev Computes the BN256 pairing check by calling the precompile at 0x08.
     * Expects matched arrays of G1 and G2 points. If success, returns true;
     * otherwise false.
     */
    function pairing(G1Point[] memory p1, G2Point[] memory p2)
        internal
        view
        returns (bool)
    {
        require(p1.length == p2.length, "pairing length mismatch");

        uint256 elements = p1.length;
        uint256 inputSize = elements * 6;
        uint256[] memory input = new uint256[](inputSize);
        uint256 counter = 0;

        for (uint256 i = 0; i < elements; i++) {
            // G1: (x, y)
            input[counter++] = p1[i].x;
            input[counter++] = p1[i].y;
            // G2: (x[0], x[1], y[0], y[1])
            input[counter++] = p2[i].x[0];
            input[counter++] = p2[i].x[1];
            input[counter++] = p2[i].y[0];
            input[counter++] = p2[i].y[1];
        }

        bool success;
        uint256 result;
        assembly {
            // staticcall into the BN256Pairing precompile (0x08)
            success := staticcall(
                gas(),
                0x08,
                add(input, 0x20),         // input data start
                mul(inputSize, 0x20),     // input length in bytes
                0x00,
                0x20
            )
            result := mload(0x00) // load the return data
        }
        return (success && result == 1);
    }

    /**
     * Pairing check for two pairs: e(p1, p2) ?= 1
     * In practice, you might chain multiple of these for a final equality check.
     */
    function pairingProd2(
        G1Point memory a1,
        G2Point memory a2,
        G1Point memory b1,
        G2Point memory b2
    ) internal view returns (bool) {
        G1Point[] memory p1 = new G1Point[](2);
        G2Point[] memory p2 = new G2Point[](2);
        p1[0] = a1;
        p1[1] = b1;
        p2[0] = a2;
        p2[1] = b2;
        return pairing(p1, p2);
    }

    /**
     * Add two G1 points via the bn256Add precompile (address 0x06).
     */
    function g1Add(G1Point memory p1, G1Point memory p2)
        internal
        view
        returns (G1Point memory r)
    {
        // The precompile expects (x1, y1, x2, y2) as 4*32 bytes.
        uint256[4] memory input;
        input[0] = p1.x;
        input[1] = p1.y;
        input[2] = p2.x;
        input[3] = p2.y;

        bool success;
        assembly {
            success := staticcall(
                gas(),
                0x06,
                input,
                0x80, // 4 * 32 bytes = 128
                r,
                0x40  // 2 * 32 bytes = 64 (the return: x, y)
            )
        }
        require(success, "g1Add failed");
        return r;
    }

    /**
     * Scalar-multiply a G1 point by s via the bn256ScalarMul precompile (0x07).
     */
    function g1Mul(G1Point memory p, uint256 s)
        internal
        view
        returns (G1Point memory r)
    {
        // The precompile expects (x, y, scalar) as 3*32 bytes.
        uint256[3] memory input;
        input[0] = p.x;
        input[1] = p.y;
        input[2] = s;

        bool success;
        assembly {
            success := staticcall(
                gas(),
                0x07,
                input,
                0x60, // 3 * 32 bytes = 96
                r,
                0x40  // 2 * 32 bytes = 64
            )
        }
        require(success, "g1Mul failed");
        return r;
    }

    /**
     * Returns the additive inverse of point p: effectively p.y -> -p.y mod FIELD_MODULUS.
     */
    function negate(G1Point memory p)
        internal
        pure
        returns (G1Point memory)
    {
        // If it's the point at infinity, return itself.
        if (p.x == 0 && p.y == 0) {
            return G1Point(0, 0);
        }
        // Otherwise, flip the y coordinate mod the field prime.
        return G1Point(p.x, FIELD_MODULUS - (p.y % FIELD_MODULUS));
    }
}

contract ZkVerifier {
    using Pairing for *;

    /**
     * VerifyingKey struct: holds alpha (G1), beta/gamma/delta (G2),
     * and gamma_abc for a *single* public input. 
     * The gamma_abc array is fixed at length=2:
     *   [0] => constant term
     *   [1] => coefficient for the input
     */
    struct VerifyingKey {
        Pairing.G1Point alpha;
        Pairing.G2Point beta;
        Pairing.G2Point gamma;
        Pairing.G2Point delta;
        // Exactly two G1 points:
        Pairing.G1Point[2] gamma_abc;
    }

    VerifyingKey vk;

    constructor(
        // alpha (G1)
        uint256 alphaX,
        uint256 alphaY,
        // beta (G2)
        uint256[2] memory betaX,
        uint256[2] memory betaY,
        // gamma (G2)
        uint256[2] memory gammaX,
        uint256[2] memory gammaY,
        // delta (G2)
        uint256[2] memory deltaX,
        uint256[2] memory deltaY,
        // gamma_abc[0] (constant term)
        uint256 gammaAbc0X,
        uint256 gammaAbc0Y,
        // gamma_abc[1] (coefficient for the single input)
        uint256 gammaAbc1X,
        uint256 gammaAbc1Y
    ) {
        // set alpha
        vk.alpha = Pairing.G1Point(alphaX, alphaY);

        // set beta
        vk.beta = Pairing.G2Point(betaX, betaY);

        // set gamma
        vk.gamma = Pairing.G2Point(gammaX, gammaY);

        // set delta
        vk.delta = Pairing.G2Point(deltaX, deltaY);

        // Assign the two gamma_abc points directly:
        vk.gamma_abc[0] = Pairing.G1Point(gammaAbc0X, gammaAbc0Y);
        vk.gamma_abc[1] = Pairing.G1Point(gammaAbc1X, gammaAbc1Y);
    }

    /**
     * verifyProof:
     *   - A  (G1) => a[0], a[1]
     *   - B  (G2) => b[0..1], b[1..1]
     *   - C  (G1) => c[0], c[1]
     *   - inputValue => the single public input
     */
    function verifyProof(
        uint256[2] memory a,
        uint256[2][2] memory b,
        uint256[2] memory c,
        uint256 inputValue
    ) public view returns (bool) {
        // Reconstruct A, B, C as G1 / G2:
        Pairing.G1Point memory A = Pairing.G1Point(a[0], a[1]);
        // b[0] is [b_x0, b_x1], b[1] is [b_y0, b_y1]
        Pairing.G2Point memory B = Pairing.G2Point(b[0], b[1]);
        Pairing.G1Point memory C = Pairing.G1Point(c[0], c[1]);

        // Perform the Groth16 pairing checks:
        return doPairingCheck(A, B, C, inputValue);
    }

    /**
     * doPairingCheck:
     *   1. Compute X = C + vk.gamma_abc[0] + (inputValue * vk.gamma_abc[1]).
     *   2. Check that e(A, B) * e(-vk.alpha, vk.beta)
     *                 * e(-X, vk.gamma) * e(-C, vk.delta) == 1.
     */
    function doPairingCheck(
        Pairing.G1Point memory A,
        Pairing.G2Point memory B,
        Pairing.G1Point memory C,
        uint256 inputValue
    ) internal view returns (bool) {
        // 1) Compute X = C + gamma_abc[0] + inputValue * gamma_abc[1].
        //    We'll use the BN256 precompiles for G1 add & mul.

        // X = gamma_abc[0] + (inputValue * gamma_abc[1])
        Pairing.G1Point memory gammaAbcInput =
            Pairing.g1Mul(vk.gamma_abc[1], inputValue);
        gammaAbcInput = Pairing.g1Add(vk.gamma_abc[0], gammaAbcInput);

        // Finally add C:
        Pairing.G1Point memory X = Pairing.g1Add(C, gammaAbcInput);

        // 2) Build arrays for the pairing check:
        //    We want: e(A, B) * e(-vk.alpha, vk.beta)
        //              * e(-X, vk.gamma) * e(-C, vk.delta) == 1.

        Pairing.G1Point[] memory g1Points = new Pairing.G1Point[](4);
        Pairing.G2Point[] memory g2Points = new Pairing.G2Point[](4);

        g1Points[0] = A;
        g2Points[0] = B;

        // e(-alpha, beta)
        g1Points[1] = Pairing.negate(vk.alpha);
        g2Points[1] = vk.beta;

        // e(-X, gamma)
        g1Points[2] = Pairing.negate(X);
        g2Points[2] = vk.gamma;

        // e(-C, delta)
        g1Points[3] = Pairing.negate(C);
        g2Points[3] = vk.delta;

        // If the product of pairings == 1, the proof is valid:
        return Pairing.pairing(g1Points, g2Points);
    }
}
