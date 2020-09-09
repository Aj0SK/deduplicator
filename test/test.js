const util = require('util');
const exec = util.promisify(require('child_process').exec);
const tap = require('tap');

const mock = x => x.split("").map(x=>Math.random()<1/2?x.toLowerCase():x.toUpperCase()).join("");

const presliTesty = false;
const fdupesTests = false;

(async () => {
    let fdupesSorted, deduplicatorSorted, cppSorted, deduplicatorDummyHashSorted;
    // you may encounter invalid number of args here, but it's actually valid as the promisify takes
    // the LAST argument as (err,cb), not second

    await (async () => {
        const {stdout, stderr} = await exec('make -s create_test_data', {cwd: "../"});
        tap.ok(stdout);
        //tap.notOk(stderr);
        cppSorted = stdout
            .split("\n")
            .filter(Boolean)
            .map(x=>x.trim().split(" "))
            .map(x=>({order:x,sorted:[...x].sort()}))
            .sort((a,b) => a.sorted.toString().localeCompare(b.sorted.toString()));
    })();
    if(fdupesTests){
        await (async () => {
            const {stdout, stderr} = await exec('fdupes -r1q data', {cwd: "../"});
            tap.ok(stdout, "Fdupes stdout should be truthy");
            console.log("Fdupes");
            console.log(stdout);
            tap.notOk(stderr, "Fdupes stderr should be falsey");
            fdupesSorted = stdout
                .split("\n")
                .filter(Boolean)
                .map(x=>x.trim().split(" "))
                .map(x=>({order:x,sorted: [...x].sort()}))
                .sort((a,b) => a.sorted.toString().localeCompare(b.sorted.toString()));
        })();

    }
    await (async () => {
        const {stdout, stderr} = await exec('make -s main-notime-nodelete', {cwd: "../"});
        tap.ok(stdout, "Deduplicator stdout should be truthy");
        console.log("deduplicator");
        console.log(stdout);
        tap.notOk(stderr, "Deduplicator stderr should be falsey");
        deduplicatorSorted = stdout
            .split("\n")
            .filter(Boolean)
            .map(x=>x.trim().split(" "))
            .map(x=>({order:x,sorted:[...x].sort()}))
            .sort((a,b) => a.sorted.toString().localeCompare(b.sorted.toString()));
    })();
    await (async () => {
        const {stdout, stderr} = await exec('make -s main-notime-nodelete-dummyhash', {cwd: "../"});
        tap.ok(stdout, "Deduplicator with dummy hash stdout should be truthy");
        console.log("deduplicator with dummy hash");
        console.log(stdout);
        tap.notOk(stderr, "Deduplicator with dummy hash stderr should be falsey");
        deduplicatorDummyHashSorted = stdout
            .split("\n")
            .filter(Boolean)
            .map(x=>x.trim().split(" "))
            .map(x=>({order:x,sorted:[...x].sort()}))
            .sort((a,b) => a.sorted.toString().localeCompare(b.sorted.toString()));
    })();
    tap.ok(deduplicatorDummyHashSorted);
    tap.ok(deduplicatorSorted);
    tap.ok(cppSorted);
    fdupesTests&&tap.same(deduplicatorSorted, fdupesSorted, "Outputs deduplicatorSorted and fdupesSorted should be equal");
    fdupesTests&&tap.same(deduplicatorDummyHashSorted, fdupesSorted, "Outputs deduplicatorDummyHashSorted and fdupesSorted should be equal");
    tap.same(cppSorted, deduplicatorSorted, "Outputs cppSorted and deduplicatorSorted should be equal");
    tap.same(cppSorted, deduplicatorDummyHashSorted, "Outputs cppSorted and deduplicatorDummyHashSorted should be equal");

    fdupesTests&&deduplicatorSorted.map((entry,index) => tap.same(entry.sorted, fdupesSorted[index].sorted, "entry items should be equal"));
    fdupesTests&&deduplicatorSorted.map((entry,index) => tap.same(entry.order, fdupesSorted[index].order, "entry order of items should be equal"));
    deduplicatorSorted.map((entry,index) => tap.same(entry.sorted, cppSorted[index].sorted, "entry items should be equal"));
    deduplicatorSorted.map((entry,index) => tap.same(entry.order, cppSorted[index].order, "entry order of items should be equal"));

    fdupesTests&&deduplicatorDummyHashSorted.map((entry,index) => tap.same(entry.sorted, fdupesSorted[index].sorted, "entry items should be equal"));
    fdupesTests&&deduplicatorDummyHashSorted.map((entry,index) => tap.same(entry.order, fdupesSorted[index].order, "entry order of items should be equal"));
    deduplicatorDummyHashSorted.map((entry,index) => tap.same(entry.sorted, cppSorted[index].sorted, "entry items should be equal"));
    deduplicatorDummyHashSorted.map((entry,index) => tap.same(entry.order, cppSorted[index].order, "entry order of items should be equal"));



})();