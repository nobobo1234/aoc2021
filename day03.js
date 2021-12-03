const fs = require('fs');

const file = fs.readFileSync('./input.txt');
const lines =
    file.toString().split('\n').map(e => e.split('').map(e => Number(e)));

let oxy = lines;
let co2 = lines;

for (let i = 0; i < 12; i++) {
    let sum =
        oxy.reduce((previous, curr) => previous + (curr[i] ? curr[i] : 0), 0);
    let common = sum >= oxy.length / 2;

    oxy = oxy.filter(e => e[i] == common);

    if (oxy.length <= 1) {
        break;
    }
}

for (let i = 0; i < 12; i++) {
    // Sum all the bits on the i'th position of every element.
    let sum =
        co2.reduce((previous, curr) => previous + (curr[i] ? curr[i] : 0), 0);
    // If the sum of all those bits is higher or equal than the total length of
    // the arraqy. 1 is the most common.
    let common = sum >= co2.length / 2;

    // Filter out all the elements that start with a 0.
    co2 = co2.filter(e => e[i] == !common);

    if (co2.length <= 1) {
        break;
    }
}

console.log(oxy[0].join(''));
console.log(co2[0].join(''))
console.log(parseInt(oxy[0].join(''), 2) * parseInt(co2[0].join(''), 2));
