pragma circom 2.0.0;

include "../../node_modules/circomlib/circuits/comparators.circom";

template AgeChecker() {
    signal input age;
    signal input limit;
    signal output isAdult;

    component geq = GreaterEqThan(7);
    
    geq.in[0] <== age;
    geq.in[1] <== limit;

    isAdult <== geq.out;
}

component main {public [limit]} = AgeChecker();
