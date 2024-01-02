class Fraction {
  numerator: number;
  denominator: number;
  iterations: number;
  decimalValue: number;

  constructor(numerator: number, denominator: number, iterations: number) {
    this.numerator = numerator;
    this.denominator = denominator;
    this.iterations = iterations;
    this.decimalValue = numerator / denominator;
  }

  toString(): string {
    const numeratorLength = this.numerator.toString().length;
    const denominatorLength = this.denominator.toString().length;
    const separatorLength = Math.max(numeratorLength, denominatorLength) + 2;
    const lineSeparator = `${this.decimalValue} ≈ ${"-".repeat(separatorLength)}`;
    const padding = ' '.repeat(this.decimalValue.toString().length + 3);
    return `\n${padding} ${this.numerator}\n${lineSeparator}\n${padding} ${this.denominator}\n`;
  }
}

function approximateFraction(target: number): Fraction {
  let lowerNumerator = Math.floor(target);
  let lowerDenominator = 1;
  let upperNumerator = Math.ceil(target);
  let upperDenominator = 1;
  let mediantNumerator = 0;
  let mediantDenominator = 1;
  let mediantValue = 0;
  let iterationCount = 0;

  do {
    mediantNumerator = lowerNumerator + upperNumerator;
    mediantDenominator = lowerDenominator + upperDenominator;
    mediantValue = mediantNumerator / mediantDenominator;

    if (mediantValue === target) break;

    if (mediantValue < target) {
      lowerNumerator = mediantNumerator;
      lowerDenominator = mediantDenominator;
    } else {
      upperNumerator = mediantNumerator;
      upperDenominator = mediantDenominator;
    }
    iterationCount++;
  } while(Math.abs(target - mediantValue) > Number.EPSILON);

  return new Fraction(mediantNumerator, mediantDenominator, iterationCount);
}

function printApproximation(target: number, fraction: Fraction) {
  console.log(`${target}\t${fraction}`);
  console.log(`${Math.abs(target - fraction.decimalValue)} delta`);
}

function main() {
  const targetValue = .68
  const approximatedFraction = approximateFraction(targetValue);
  printApproximation(targetValue, approximatedFraction);
}

if (require.main === module) {
  main();
}
