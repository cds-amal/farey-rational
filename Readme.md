## Rational number approximation

Approximates a real number `x` using the Farey sequence method. It returns a `Fraction` object representing the closest approximation it can find within machine precision.

- The Farey sequence is used to find fractions close to a given real number.
- It iteratively finds the mediant of two fractions (the numerator and denominator of the mediant are the sums of the numerators and denominators of the two fractions, respectively) until it gets close enough to the target number.
- The mediant is a concept in mathematics, particularly in number theory, where it is defined as the fraction `(a+c)/(b+d)` for two fractions `a/b` and `c/d`.


For more information about the mediant and the Farey sequence, you might find these resources helpful:
- [Mediant on Wikipedia](https://en.wikipedia.org/wiki/Mediant_(mathematics))
- [Farey sequence on Wikipedia](https://en.wikipedia.org/wiki/Farey_sequence)



### Rust

This solution reads from the command line.

```console
cd rs
cargo run -- --number .547
```


### TypeScript

This solution hard codes the fixed point number to approximate; change the value in the code
```console
cd ts
npm ci
npm run test
```
