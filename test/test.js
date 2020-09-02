const util = require('util');
const exec = util.promisify(require('child_process').exec);
const tap = require('tap');

const mock = x => x.split("").map(x=>Math.random()<1/2?x.toLowerCase():x.toUpperCase()).join("");


(async () => {
    let fdupesSorted, deduplicatorSorted;
    // you may encounter invalid number of args here, but it's actually valid as the promisify takes
    // the LAST argument as (err,cb), not second

    await (async () => {
        const {stdout, stderr} = await exec('make create_test_data', {cwd: "../"});
        tap.ok(stdout);
    })();
    await (async () => {
        const {stdout, stderr} = await exec('fdupes -r1q data', {cwd: "../"});
        tap.ok(stdout, "Fdupes stdout should be truthy");
        tap.notOk(stderr, "Fdupes stderr should be falsey");
        fdupesSorted = stdout.split("\n").filter(Boolean).map(x=>x.trim()).sort();
    })();
    await (async () => {
        const {stdout, stderr} = await exec('make -s main-notime', {cwd: "../"});
        tap.ok(stdout, "Deduplicator stdout should be truthy");
        tap.notOk(stderr, "Deduplicator stderr should be falsey");
        deduplicatorSorted = stdout.split("\n").filter(Boolean).sort();
    })();
    tap.ok(deduplicatorSorted);
    tap.same(deduplicatorSorted, fdupesSorted, "Outputs should be equal");

    fdupesSorted.map((entry,index) => tap.same(entry, deduplicatorSorted[index], "entry should be equal"));


})();