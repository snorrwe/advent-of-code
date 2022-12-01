const md5 = require('md5');

function solve(base){
    let i=0;
    let result = [];
    while(result.length < 8){
        const input = base + i;
        const hash = md5(input);
        if (hash.startsWith('00000')) {
            result.push(hash[5]);
        }
        i += 1;
    }
    return result.join('');


}

console.log(solve('abc'))
console.log(solve('cxdnnyjw'))
