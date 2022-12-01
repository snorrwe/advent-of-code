const md5 = require("md5");

function solve(base) {
  let i = 0;
  let p1 = [];
  let p2 = [
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
  ];
  while (p2.some((x) => x === undefined)) {
    const input = base + i;
    const hash = md5(input);
    if (hash.startsWith("00000")) {
      let idx = hash[5];

      if (p1.length < 8) p1.push(idx);

      idx = parseInt(idx);
      if (!isNaN(idx) && idx < 8 && !p2[idx]) {
        p2[idx] = hash[6];
      }
    }
    i += 1;
  }
  return [p1.join(""), p2.join("")].join("  ");
}

console.log(solve("abc"));
console.log(solve("cxdnnyjw"));
