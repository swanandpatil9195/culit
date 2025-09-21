use culit::culit;

#[culit]
fn float() {
    70.0f16;
    70.0f128;
}

#[culit]
fn int() {
    100i256;
    70.0u256;
}
