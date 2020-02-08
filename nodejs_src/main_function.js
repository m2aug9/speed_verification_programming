console.time('nodejs12');
let number = 0;
let pi = 0;
while(number <= 100000000){
    pi = pi + ((-1) ** number * 1 / (2 * number + 1));
    number = number + 1;
}
console.log('pi = ' + pi * 4);
console.timeEnd('nodejs12');