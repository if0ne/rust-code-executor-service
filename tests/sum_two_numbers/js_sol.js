const readline = require('readline');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});


rl.question("", (a) => {
    rl.question("", (b) => {
        console.log((+a) + (+b));
        rl.close();
    })
});