const util = require('util');
const exec = util.promisify(require('child_process').exec);
const tap = require('tap');

(async () => {
    // you may encounter invalid number of args here, but it's actually valid as the promisify takes
    // the LAST argument as (err,cb), not second

    await (async () => {
        const {stdout, stderr} = await exec('make create_test_data', {cwd: "../"});
        console.log(stdout);
        tap.ok(stdout);
    })();
    await (async () => {
        const {stdout, stderr} = await exec('make main', {cwd: "../"});
        console.log(stdout);
        tap.ok(stdout);
    })();


})();