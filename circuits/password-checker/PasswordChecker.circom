pragma circom 2.0.0;

include "../../node_modules/circomlib/circuits/poseidon.circom";
include "../../node_modules/circomlib/circuits/comparators.circom"; 

template PasswordChecker() {
    signal input password;
    signal input storedHash;
    signal output isValid;

    component hasher = Poseidon(1);
    hasher.inputs[0] <== password;

    component eq = IsEqual();
    eq.in[0] <== hasher.out;
    eq.in[1] <== storedHash;

    isValid <== eq.out;
}

component main {public [storedHash]} = PasswordChecker();
