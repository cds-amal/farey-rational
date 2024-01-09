use bpaf::Bpaf;
#[macro_use] extern crate impl_ops;

mod fraction;
use fraction::Fraction;

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]

struct Args {
    /// The rational/irrational number
    number: f64,
}

fn farey(target: f64) -> Fraction {
    let mut left = Fraction::new(target.floor() as u64, 1u64);
    let mut right = Fraction::new(target.ceil() as u64, 1u64);

    let mut mediant: Fraction;
    let e = f64::EPSILON; // * 1e5;

    loop {
        mediant = &left + &right;

        if mediant.value == target || (target - mediant.value).abs() <= e {
            break;
        }

        // println!("$ frac({},{}) <- {} -> frac({},{}) {} {} $",
        //     left.numerator, left.denominator,
        //     mediant.value,
        //     right.numerator, right.denominator,
        //     (f64::EPSILON - (target - mediant.value)).abs(),
        //     f64::EPSILON
        // );

        if mediant.value > target {
            right = mediant;
        } else {
            left = mediant;
        }
    }

    println!("$ frac({},{}) <- {} -> frac({},{}) $",
        left.numerator, left.denominator,
        mediant.value,
        right.numerator, right.denominator
    );


    mediant
}

fn main() {
    let opts = args().run();
    let approx = farey(opts.number);
    println!("{}", approx);
    println!("{}: delta: {}", approx.value, (opts.number - approx.value).abs());
}
